/* FP:linker.rs-0001 */ use std::ffi::{OsStr, OsString};
/* FP:linker.rs-0002 */ use std::fs::{self, File};
/* FP:linker.rs-0003 */ use std::io::prelude::*;
/* FP:linker.rs-0004 */ use std::path::{Path, PathBuf};
/* FP:linker.rs-0005 */ use std::{env, io, iter, mem, str};
/* FP:linker.rs-0006 */ 
/* FP:linker.rs-0007 */ use cc::windows_registry;
/* FP:linker.rs-0008 */ use crate::rustc_complete::def_id::{CrateNum, LOCAL_CRATE};
/* FP:linker.rs-0009 */ use crate::rustc_metadata::{
/* FP:linker.rs-0010 */     find_native_static_library, try_find_native_dynamic_library, try_find_native_static_library,
/* FP:linker.rs-0011 */ };
/* FP:linker.rs-0012 */ use crate::rustc_complete::bug;
/* FP:linker.rs-0013 */ use crate::rustc_complete::middle::dependency_format::Linkage;
/* FP:linker.rs-0014 */ use crate::rustc_complete::middle::exported_symbols::{
/* FP:linker.rs-0015 */     self, ExportedSymbol, SymbolExportInfo, SymbolExportKind, SymbolExportLevel,
/* FP:linker.rs-0016 */ };
/* FP:linker.rs-0017 */ use crate::rustc_complete::ty::TyCtxt;
/* FP:linker.rs-0018 */ use crate::rustc_complete::Session;
/* FP:linker.rs-0019 */ use crate::rustc_complete::config::{self, CrateType, DebugInfo, LinkerPluginLto, Lto, OptLevel, Strip};
/* FP:linker.rs-0020 */ use crate::rustc_complete::sym;
/* FP:linker.rs-0021 */ use crate::rustc_target::spec::{Cc, LinkOutputKind, LinkerFlavor, Lld};
/* FP:linker.rs-0022 */ use tracing::{debug, warn};
/* FP:linker.rs-0023 */ 
/* FP:linker.rs-0024 */ use super::command::Command;
/* FP:linker.rs-0025 */ use super::symbol_export;
/* FP:linker.rs-0026 */ use crate::back::symbol_export::allocator_shim_symbols;
/* FP:linker.rs-0027 */ use crate::base::needs_allocator_shim_for_linking;
/* FP:linker.rs-0028 */ use crate::errors;
/* FP:linker.rs-0029 */ 
/* FP:linker.rs-0030 */ #[cfg(test)]
/* FP:linker.rs-0032 */ 
/* FP:linker.rs-0033 */ /// Disables non-English messages from localized linkers.
/* FP:linker.rs-0034 */ /// Such messages may cause issues with text encoding on Windows (#35785)
/* FP:linker.rs-0035 */ /// and prevent inspection of linker output in case of errors, which we occasionally do.
/* FP:linker.rs-0036 */ /// This should be acceptable because other messages from rustc are in English anyway,
/* FP:linker.rs-0037 */ /// and may also be desirable to improve searchability of the linker diagnostics.
/* FP:linker.rs-0038 */ pub(crate) fn disable_localization(linker: &mut Command) {
/* FP:linker.rs-0039 */     // No harm in setting both env vars simultaneously.
/* FP:linker.rs-0040 */     // Unix-style linkers.
/* FP:linker.rs-0041 */     linker.env("LC_ALL", "C");
/* FP:linker.rs-0042 */     // MSVC's `link.exe`.
/* FP:linker.rs-0043 */     linker.env("VSLANG", "1033");
/* FP:linker.rs-0044 */ }
/* FP:linker.rs-0045 */ 
/* FP:linker.rs-0046 */ /// The third parameter is for env vars, used on windows to set up the
/* FP:linker.rs-0047 */ /// path for MSVC to find its DLLs, and gcc to find its bundled
/* FP:linker.rs-0048 */ /// toolchain
/* FP:linker.rs-0049 */ pub(crate) fn get_linker<'a>(
/* FP:linker.rs-0050 */     sess: &'a Session,
/* FP:linker.rs-0051 */     linker: &Path,
/* FP:linker.rs-0052 */     flavor: LinkerFlavor,
/* FP:linker.rs-0053 */     self_contained: bool,
/* FP:linker.rs-0054 */     target_cpu: &'a str,
/* FP:linker.rs-0055 */ ) -> Box<dyn Linker + 'a> {
/* FP:linker.rs-0056 */     let msvc_tool = windows_registry::find_tool(&sess.target.arch, "link.exe");
/* FP:linker.rs-0057 */ 
/* FP:linker.rs-0058 */     // If our linker looks like a batch script on Windows then to execute this
/* FP:linker.rs-0059 */     // we'll need to spawn `cmd` explicitly. This is primarily done to handle
/* FP:linker.rs-0060 */     // emscripten where the linker is `emcc.bat` and needs to be spawned as
/* FP:linker.rs-0061 */     // `cmd /c emcc.bat ...`.
/* FP:linker.rs-0062 */     //
/* FP:linker.rs-0063 */     // This worked historically but is needed manually since #42436 (regression
/* FP:linker.rs-0064 */     // was tagged as #42791) and some more info can be found on #44443 for
/* FP:linker.rs-0065 */     // emscripten itself.
/* FP:linker.rs-0066 */     let mut cmd = match linker.to_str() {
/* FP:linker.rs-0067 */         Some(linker) if cfg!(windows) && linker.ends_with(".bat") => Command::bat_script(linker),
/* FP:linker.rs-0068 */         _ => match flavor {
/* FP:linker.rs-0069 */             LinkerFlavor::Gnu(Cc::No, Lld::Yes)
/* FP:linker.rs-0070 */             | LinkerFlavor::Darwin(Cc::No, Lld::Yes)
/* FP:linker.rs-0071 */             | LinkerFlavor::WasmLld(Cc::No)
/* FP:linker.rs-0072 */             | LinkerFlavor::Msvc(Lld::Yes) => Command::lld(linker, flavor.lld_flavor()),
/* FP:linker.rs-0073 */             LinkerFlavor::Msvc(Lld::No)
/* FP:linker.rs-0074 */                 if sess.opts.cg.linker.is_none() && sess.target.linker.is_none() =>
/* FP:linker.rs-0075 */             {
/* FP:linker.rs-0076 */                 Command::new(msvc_tool.as_ref().map_or(linker, |t| t.path()))
/* FP:linker.rs-0077 */             }
/* FP:linker.rs-0078 */             _ => Command::new(linker),
/* FP:linker.rs-0079 */         },
/* FP:linker.rs-0080 */     };
/* FP:linker.rs-0081 */ 
/* FP:linker.rs-0082 */     // UWP apps have API restrictions enforced during Store submissions.
/* FP:linker.rs-0083 */     // To comply with the Windows App Certification Kit,
/* FP:linker.rs-0084 */     // MSVC needs to link with the Store versions of the runtime libraries (vcruntime, msvcrt, etc).
/* FP:linker.rs-0085 */     let t = &sess.target;
/* FP:linker.rs-0086 */     if matches!(flavor, LinkerFlavor::Msvc(..)) && t.vendor == "uwp" {
/* FP:linker.rs-0087 */         if let Some(ref tool) = msvc_tool {
/* FP:linker.rs-0088 */             let original_path = tool.path();
/* FP:linker.rs-0089 */             if let Some(root_lib_path) = original_path.ancestors().nth(4) {
/* FP:linker.rs-0090 */                 let arch = match t.arch.as_ref() {
/* FP:linker.rs-0091 */                     "x86_64" => Some("x64"),
/* FP:linker.rs-0092 */                     "x86" => Some("x86"),
/* FP:linker.rs-0093 */                     "aarch64" => Some("arm64"),
/* FP:linker.rs-0094 */                     "arm" => Some("arm"),
/* FP:linker.rs-0095 */                     _ => None,
/* FP:linker.rs-0096 */                 };
/* FP:linker.rs-0097 */                 if let Some(ref a) = arch {
/* FP:linker.rs-0098 */                     // FIXME: Move this to `fn linker_with_args`.
/* FP:linker.rs-0099 */                     let mut arg = OsString::from("/LIBPATH:");
/* FP:linker.rs-0100 */                     arg.push(format!("{}\\lib\\{}\\store", root_lib_path.display(), a));
/* FP:linker.rs-0101 */                     cmd.arg(&arg);
/* FP:linker.rs-0102 */                 } else {
/* FP:linker.rs-0103 */                     warn!("arch is not supported");
/* FP:linker.rs-0104 */                 }
/* FP:linker.rs-0105 */             } else {
/* FP:linker.rs-0106 */                 warn!("MSVC root path lib location not found");
/* FP:linker.rs-0107 */             }
/* FP:linker.rs-0108 */         } else {
/* FP:linker.rs-0109 */             warn!("link.exe not found");
/* FP:linker.rs-0110 */         }
/* FP:linker.rs-0111 */     }
/* FP:linker.rs-0112 */ 
/* FP:linker.rs-0113 */     // The compiler's sysroot often has some bundled tools, so add it to the
/* FP:linker.rs-0114 */     // PATH for the child.
/* FP:linker.rs-0115 */     let mut new_path = sess.get_tools_search_paths(self_contained);
/* FP:linker.rs-0116 */     let mut msvc_changed_path = false;
/* FP:linker.rs-0117 */     if sess.target.is_like_msvc
/* FP:linker.rs-0118 */         && let Some(ref tool) = msvc_tool
/* FP:linker.rs-0119 */     {
/* FP:linker.rs-0120 */         cmd.args(tool.args());
/* FP:linker.rs-0121 */         for (k, v) in tool.env() {
/* FP:linker.rs-0122 */             if k == "PATH" {
/* FP:linker.rs-0123 */                 new_path.extend(env::split_paths(v));
/* FP:linker.rs-0124 */                 msvc_changed_path = true;
/* FP:linker.rs-0125 */             } else {
/* FP:linker.rs-0126 */                 cmd.env(k, v);
/* FP:linker.rs-0127 */             }
/* FP:linker.rs-0128 */         }
/* FP:linker.rs-0129 */     }
/* FP:linker.rs-0130 */ 
/* FP:linker.rs-0131 */     if !msvc_changed_path && let Some(path) = env::var_os("PATH") {
/* FP:linker.rs-0132 */         new_path.extend(env::split_paths(&path));
/* FP:linker.rs-0133 */     }
/* FP:linker.rs-0134 */     cmd.env("PATH", env::join_paths(new_path).unwrap());
/* FP:linker.rs-0135 */ 
/* FP:linker.rs-0136 */     // FIXME: Move `/LIBPATH` addition for uwp targets from the linker construction
/* FP:linker.rs-0137 */     // to the linker args construction.
/* FP:linker.rs-0138 */     assert!(cmd.get_args().is_empty() || sess.target.vendor == "uwp");
/* FP:linker.rs-0139 */     match flavor {
/* FP:linker.rs-0140 */         LinkerFlavor::Unix(Cc::No) if sess.target.os == "l4re" => {
/* FP:linker.rs-0141 */             Box::new(L4Bender::new(cmd, sess)) as Box<dyn Linker>
/* FP:linker.rs-0142 */         }
/* FP:linker.rs-0143 */         LinkerFlavor::Unix(Cc::No) if sess.target.os == "aix" => {
/* FP:linker.rs-0144 */             Box::new(AixLinker::new(cmd, sess)) as Box<dyn Linker>
/* FP:linker.rs-0145 */         }
/* FP:linker.rs-0146 */         LinkerFlavor::WasmLld(Cc::No) => Box::new(WasmLd::new(cmd, sess)) as Box<dyn Linker>,
/* FP:linker.rs-0147 */         LinkerFlavor::Gnu(cc, _)
/* FP:linker.rs-0148 */         | LinkerFlavor::Darwin(cc, _)
/* FP:linker.rs-0149 */         | LinkerFlavor::WasmLld(cc)
/* FP:linker.rs-0150 */         | LinkerFlavor::Unix(cc) => Box::new(GccLinker {
/* FP:linker.rs-0151 */             cmd,
/* FP:linker.rs-0152 */             sess,
/* FP:linker.rs-0153 */             target_cpu,
/* FP:linker.rs-0154 */             hinted_static: None,
/* FP:linker.rs-0155 */             is_ld: cc == Cc::No,
/* FP:linker.rs-0156 */             is_gnu: flavor.is_gnu(),
/* FP:linker.rs-0157 */             uses_lld: flavor.uses_lld(),
/* FP:linker.rs-0158 */         }) as Box<dyn Linker>,
/* FP:linker.rs-0159 */         LinkerFlavor::Msvc(..) => Box::new(MsvcLinker { cmd, sess }) as Box<dyn Linker>,
/* FP:linker.rs-0160 */         LinkerFlavor::EmCc => Box::new(EmLinker { cmd, sess }) as Box<dyn Linker>,
/* FP:linker.rs-0161 */         LinkerFlavor::Bpf => Box::new(BpfLinker { cmd, sess }) as Box<dyn Linker>,
/* FP:linker.rs-0162 */         LinkerFlavor::Llbc => Box::new(LlbcLinker { cmd, sess }) as Box<dyn Linker>,
/* FP:linker.rs-0163 */         LinkerFlavor::Ptx => Box::new(PtxLinker { cmd, sess }) as Box<dyn Linker>,
/* FP:linker.rs-0164 */     }
/* FP:linker.rs-0165 */ }
/* FP:linker.rs-0166 */ 
/* FP:linker.rs-0167 */ // Note: Ideally neither these helper function, nor the macro-generated inherent methods below
/* FP:linker.rs-0168 */ // would exist, and these functions would live in `trait Linker`.
/* FP:linker.rs-0169 */ // Unfortunately, adding these functions to `trait Linker` make it `dyn`-incompatible.
/* FP:linker.rs-0170 */ // If the methods are added to the trait with `where Self: Sized` bounds, then even a separate
/* FP:linker.rs-0171 */ // implementation of them for `dyn Linker {}` wouldn't work due to a conflict with those
/* FP:linker.rs-0172 */ // uncallable methods in the trait.
/* FP:linker.rs-0173 */ 
/* FP:linker.rs-0174 */ /// Just pass the arguments to the linker as is.
/* FP:linker.rs-0175 */ /// It is assumed that they are correctly prepared in advance.
/* FP:linker.rs-0176 */ fn verbatim_args<L: Linker + ?Sized>(
/* FP:linker.rs-0177 */     l: &mut L,
/* FP:linker.rs-0178 */     args: impl IntoIterator<Item: AsRef<OsStr>>,
/* FP:linker.rs-0179 */ ) -> &mut L {
/* FP:linker.rs-0180 */     for arg in args {
/* FP:linker.rs-0181 */         l.cmd().arg(arg);
/* FP:linker.rs-0182 */     }
/* FP:linker.rs-0183 */     l
/* FP:linker.rs-0184 */ }
/* FP:linker.rs-0185 */ /// Add underlying linker arguments to C compiler command, by wrapping them in
/* FP:linker.rs-0186 */ /// `-Wl` or `-Xlinker`.
/* FP:linker.rs-0187 */ fn convert_link_args_to_cc_args(cmd: &mut Command, args: impl IntoIterator<Item: AsRef<OsStr>>) {
/* FP:linker.rs-0188 */     let mut combined_arg = OsString::from("-Wl");
/* FP:linker.rs-0189 */     for arg in args {
/* FP:linker.rs-0190 */         // If the argument itself contains a comma, we need to emit it
/* FP:linker.rs-0191 */         // as `-Xlinker`, otherwise we can use `-Wl`.
/* FP:linker.rs-0192 */         if arg.as_ref().as_encoded_bytes().contains(&b',') {
/* FP:linker.rs-0193 */             // Emit current `-Wl` argument, if any has been built.
/* FP:linker.rs-0194 */             if combined_arg != OsStr::new("-Wl") {
/* FP:linker.rs-0195 */                 cmd.arg(combined_arg);
/* FP:linker.rs-0196 */                 // Begin next `-Wl` argument.
/* FP:linker.rs-0197 */                 combined_arg = OsString::from("-Wl");
/* FP:linker.rs-0198 */             }
/* FP:linker.rs-0199 */ 
/* FP:linker.rs-0200 */             // Emit `-Xlinker` argument.
/* FP:linker.rs-0201 */             cmd.arg("-Xlinker");
/* FP:linker.rs-0202 */             cmd.arg(arg);
/* FP:linker.rs-0203 */         } else {
/* FP:linker.rs-0204 */             // Append to `-Wl` argument.
/* FP:linker.rs-0205 */             combined_arg.push(",");
/* FP:linker.rs-0206 */             combined_arg.push(arg);
/* FP:linker.rs-0207 */         }
/* FP:linker.rs-0208 */     }
/* FP:linker.rs-0209 */     // Emit final `-Wl` argument.
/* FP:linker.rs-0210 */     if combined_arg != OsStr::new("-Wl") {
/* FP:linker.rs-0211 */         cmd.arg(combined_arg);
/* FP:linker.rs-0212 */     }
/* FP:linker.rs-0213 */ }
/* FP:linker.rs-0214 */ /// Arguments for the underlying linker.
/* FP:linker.rs-0215 */ /// Add options to pass them through cc wrapper if `Linker` is a cc wrapper.
/* FP:linker.rs-0216 */ fn link_args<L: Linker + ?Sized>(l: &mut L, args: impl IntoIterator<Item: AsRef<OsStr>>) -> &mut L {
/* FP:linker.rs-0217 */     if !l.is_cc() {
/* FP:linker.rs-0218 */         verbatim_args(l, args);
/* FP:linker.rs-0219 */     } else {
/* FP:linker.rs-0220 */         convert_link_args_to_cc_args(l.cmd(), args);
/* FP:linker.rs-0221 */     }
/* FP:linker.rs-0222 */     l
/* FP:linker.rs-0223 */ }
/* FP:linker.rs-0224 */ /// Arguments for the cc wrapper specifically.
/* FP:linker.rs-0225 */ /// Check that it's indeed a cc wrapper and pass verbatim.
/* FP:linker.rs-0226 */ fn cc_args<L: Linker + ?Sized>(l: &mut L, args: impl IntoIterator<Item: AsRef<OsStr>>) -> &mut L {
/* FP:linker.rs-0227 */     assert!(l.is_cc());
/* FP:linker.rs-0228 */     verbatim_args(l, args)
/* FP:linker.rs-0229 */ }
/* FP:linker.rs-0230 */ /// Arguments supported by both underlying linker and cc wrapper, pass verbatim.
/* FP:linker.rs-0231 */ fn link_or_cc_args<L: Linker + ?Sized>(
/* FP:linker.rs-0232 */     l: &mut L,
/* FP:linker.rs-0233 */     args: impl IntoIterator<Item: AsRef<OsStr>>,
/* FP:linker.rs-0234 */ ) -> &mut L {
/* FP:linker.rs-0235 */     verbatim_args(l, args)
/* FP:linker.rs-0236 */ }
/* FP:linker.rs-0237 */ 
/* FP:linker.rs-0238 */ macro_rules! generate_arg_methods {
/* FP:linker.rs-0239 */     ($($ty:ty)*) => { $(
/* FP:linker.rs-0240 */         impl $ty {
/* FP:linker.rs-0241 */             #[allow(unused)]
/* FP:linker.rs-0242 */             pub(crate) fn verbatim_args(&mut self, args: impl IntoIterator<Item: AsRef<OsStr>>) -> &mut Self {
/* FP:linker.rs-0243 */                 verbatim_args(self, args)
/* FP:linker.rs-0244 */             }
/* FP:linker.rs-0245 */             #[allow(unused)]
/* FP:linker.rs-0246 */             pub(crate) fn verbatim_arg(&mut self, arg: impl AsRef<OsStr>) -> &mut Self {
/* FP:linker.rs-0247 */                 verbatim_args(self, iter::once(arg))
/* FP:linker.rs-0248 */             }
/* FP:linker.rs-0249 */             #[allow(unused)]
/* FP:linker.rs-0250 */             pub(crate) fn link_args(&mut self, args: impl IntoIterator<Item: AsRef<OsStr>>) -> &mut Self {
/* FP:linker.rs-0251 */                 link_args(self, args)
/* FP:linker.rs-0252 */             }
/* FP:linker.rs-0253 */             #[allow(unused)]
/* FP:linker.rs-0254 */             pub(crate) fn link_arg(&mut self, arg: impl AsRef<OsStr>) -> &mut Self {
/* FP:linker.rs-0255 */                 link_args(self, iter::once(arg))
/* FP:linker.rs-0256 */             }
/* FP:linker.rs-0257 */             #[allow(unused)]
/* FP:linker.rs-0258 */             pub(crate) fn cc_args(&mut self, args: impl IntoIterator<Item: AsRef<OsStr>>) -> &mut Self {
/* FP:linker.rs-0259 */                 cc_args(self, args)
/* FP:linker.rs-0260 */             }
/* FP:linker.rs-0261 */             #[allow(unused)]
/* FP:linker.rs-0262 */             pub(crate) fn cc_arg(&mut self, arg: impl AsRef<OsStr>) -> &mut Self {
/* FP:linker.rs-0263 */                 cc_args(self, iter::once(arg))
/* FP:linker.rs-0264 */             }
/* FP:linker.rs-0265 */             #[allow(unused)]
/* FP:linker.rs-0266 */             pub(crate) fn link_or_cc_args(&mut self, args: impl IntoIterator<Item: AsRef<OsStr>>) -> &mut Self {
/* FP:linker.rs-0267 */                 link_or_cc_args(self, args)
/* FP:linker.rs-0268 */             }
/* FP:linker.rs-0269 */             #[allow(unused)]
/* FP:linker.rs-0270 */             pub(crate) fn link_or_cc_arg(&mut self, arg: impl AsRef<OsStr>) -> &mut Self {
/* FP:linker.rs-0271 */                 link_or_cc_args(self, iter::once(arg))
/* FP:linker.rs-0272 */             }
/* FP:linker.rs-0273 */         }
/* FP:linker.rs-0274 */     )* }
/* FP:linker.rs-0275 */ }
/* FP:linker.rs-0276 */ 
/* FP:linker.rs-0277 */ generate_arg_methods! {
/* FP:linker.rs-0278 */     GccLinker<'_>
/* FP:linker.rs-0279 */     MsvcLinker<'_>
/* FP:linker.rs-0280 */     EmLinker<'_>
/* FP:linker.rs-0281 */     WasmLd<'_>
/* FP:linker.rs-0282 */     L4Bender<'_>
/* FP:linker.rs-0283 */     AixLinker<'_>
/* FP:linker.rs-0284 */     LlbcLinker<'_>
/* FP:linker.rs-0285 */     PtxLinker<'_>
/* FP:linker.rs-0286 */     BpfLinker<'_>
/* FP:linker.rs-0287 */     dyn Linker + '_
/* FP:linker.rs-0288 */ }
/* FP:linker.rs-0289 */ 
/* FP:linker.rs-0290 */ /// Linker abstraction used by `back::link` to build up the command to invoke a
/* FP:linker.rs-0291 */ /// linker.
/* FP:linker.rs-0292 */ ///
/* FP:linker.rs-0293 */ /// This trait is the total list of requirements needed by `back::link` and
/* FP:linker.rs-0294 */ /// represents the meaning of each option being passed down. This trait is then
/* FP:linker.rs-0295 */ /// used to dispatch on whether a GNU-like linker (generally `ld.exe`) or an
/* FP:linker.rs-0296 */ /// MSVC linker (e.g., `link.exe`) is being used.
/* FP:linker.rs-0297 */ pub(crate) trait Linker {
/* FP:linker.rs-0298 */     fn cmd(&mut self) -> &mut Command;
/* FP:linker.rs-0299 */     fn is_cc(&self) -> bool {
/* FP:linker.rs-0300 */         false
/* FP:linker.rs-0301 */     }
/* FP:linker.rs-0302 */     fn set_output_kind(
/* FP:linker.rs-0303 */         &mut self,
/* FP:linker.rs-0304 */         output_kind: LinkOutputKind,
/* FP:linker.rs-0305 */         crate_type: CrateType,
/* FP:linker.rs-0306 */         out_filename: &Path,
/* FP:linker.rs-0307 */     );
/* FP:linker.rs-0308 */     fn link_dylib_by_name(&mut self, _name: &str, _verbatim: bool, _as_needed: bool) {
/* FP:linker.rs-0309 */         bug!("dylib linked with unsupported linker")
/* FP:linker.rs-0310 */     }
/* FP:linker.rs-0311 */     fn link_dylib_by_path(&mut self, _path: &Path, _as_needed: bool) {
/* FP:linker.rs-0312 */         bug!("dylib linked with unsupported linker")
/* FP:linker.rs-0313 */     }
/* FP:linker.rs-0314 */     fn link_framework_by_name(&mut self, _name: &str, _verbatim: bool, _as_needed: bool) {
/* FP:linker.rs-0315 */         bug!("framework linked with unsupported linker")
/* FP:linker.rs-0316 */     }
/* FP:linker.rs-0317 */     fn link_staticlib_by_name(&mut self, name: &str, verbatim: bool, whole_archive: bool);
/* FP:linker.rs-0318 */     fn link_staticlib_by_path(&mut self, path: &Path, whole_archive: bool);
/* FP:linker.rs-0319 */     fn include_path(&mut self, path: &Path) {
/* FP:linker.rs-0320 */         link_or_cc_args(link_or_cc_args(self, &["-L"]), &[path]);
/* FP:linker.rs-0321 */     }
/* FP:linker.rs-0322 */     fn framework_path(&mut self, _path: &Path) {
/* FP:linker.rs-0323 */         bug!("framework path set with unsupported linker")
/* FP:linker.rs-0324 */     }
/* FP:linker.rs-0325 */     fn output_filename(&mut self, path: &Path) {
/* FP:linker.rs-0326 */         link_or_cc_args(link_or_cc_args(self, &["-o"]), &[path]);
/* FP:linker.rs-0327 */     }
/* FP:linker.rs-0328 */     fn add_object(&mut self, path: &Path) {
/* FP:linker.rs-0329 */         link_or_cc_args(self, &[path]);
/* FP:linker.rs-0330 */     }
/* FP:linker.rs-0331 */     fn gc_sections(&mut self, keep_metadata: bool);
/* FP:linker.rs-0332 */     fn full_relro(&mut self);
/* FP:linker.rs-0333 */     fn partial_relro(&mut self);
/* FP:linker.rs-0334 */     fn no_relro(&mut self);
/* FP:linker.rs-0335 */     fn optimize(&mut self);
/* FP:linker.rs-0336 */     fn pgo_gen(&mut self);
/* FP:linker.rs-0337 */     fn control_flow_guard(&mut self);
/* FP:linker.rs-0338 */     fn ehcont_guard(&mut self);
/* FP:linker.rs-0339 */     fn debuginfo(&mut self, strip: Strip, natvis_debugger_visualizers: &[PathBuf]);
/* FP:linker.rs-0340 */     fn no_crt_objects(&mut self);
/* FP:linker.rs-0341 */     fn no_default_libraries(&mut self);
/* FP:linker.rs-0342 */     fn export_symbols(
/* FP:linker.rs-0343 */         &mut self,
/* FP:linker.rs-0344 */         tmpdir: &Path,
/* FP:linker.rs-0345 */         crate_type: CrateType,
/* FP:linker.rs-0346 */         symbols: &[(String, SymbolExportKind)],
/* FP:linker.rs-0347 */     );
/* FP:linker.rs-0348 */     fn subsystem(&mut self, subsystem: &str);
/* FP:linker.rs-0349 */     fn linker_plugin_lto(&mut self);
/* FP:linker.rs-0350 */     fn add_eh_frame_header(&mut self) {}
/* FP:linker.rs-0351 */     fn add_no_exec(&mut self) {}
/* FP:linker.rs-0352 */     fn add_as_needed(&mut self) {}
/* FP:linker.rs-0353 */     fn reset_per_library_state(&mut self) {}
/* FP:linker.rs-0354 */ }
/* FP:linker.rs-0355 */ 
/* FP:linker.rs-0356 */ impl dyn Linker + '_ {
/* FP:linker.rs-0357 */     pub(crate) fn take_cmd(&mut self) -> Command {
/* FP:linker.rs-0358 */         mem::replace(self.cmd(), Command::new(""))
/* FP:linker.rs-0359 */     }
/* FP:linker.rs-0360 */ }
/* FP:linker.rs-0361 */ 
/* FP:linker.rs-0362 */ struct GccLinker<'a> {
/* FP:linker.rs-0363 */     cmd: Command,
/* FP:linker.rs-0364 */     sess: &'a Session,
/* FP:linker.rs-0365 */     target_cpu: &'a str,
/* FP:linker.rs-0366 */     hinted_static: Option<bool>, // Keeps track of the current hinting mode.
/* FP:linker.rs-0367 */     // Link as ld
/* FP:linker.rs-0368 */     is_ld: bool,
/* FP:linker.rs-0369 */     is_gnu: bool,
/* FP:linker.rs-0370 */     uses_lld: bool,
/* FP:linker.rs-0371 */ }
/* FP:linker.rs-0372 */ 
/* FP:linker.rs-0373 */ impl<'a> GccLinker<'a> {
/* FP:linker.rs-0374 */     fn takes_hints(&self) -> bool {
/* FP:linker.rs-0375 */         // Really this function only returns true if the underlying linker
/* FP:linker.rs-0376 */         // configured for a compiler is binutils `ld.bfd` and `ld.gold`. We
/* FP:linker.rs-0377 */         // don't really have a foolproof way to detect that, so rule out some
/* FP:linker.rs-0378 */         // platforms where currently this is guaranteed to *not* be the case:
/* FP:linker.rs-0379 */         //
/* FP:linker.rs-0380 */         // * On OSX they have their own linker, not binutils'
/* FP:linker.rs-0381 */         // * For WebAssembly the only functional linker is LLD, which doesn't
/* FP:linker.rs-0382 */         //   support hint flags
/* FP:linker.rs-0383 */         !self.sess.target.is_like_darwin && !self.sess.target.is_like_wasm
/* FP:linker.rs-0384 */     }
/* FP:linker.rs-0385 */ 
/* FP:linker.rs-0386 */     // Some platforms take hints about whether a library is static or dynamic.
/* FP:linker.rs-0387 */     // For those that support this, we ensure we pass the option if the library
/* FP:linker.rs-0388 */     // was flagged "static" (most defaults are dynamic) to ensure that if
/* FP:linker.rs-0389 */     // libfoo.a and libfoo.so both exist that the right one is chosen.
/* FP:linker.rs-0390 */     fn hint_static(&mut self) {
/* FP:linker.rs-0391 */         if !self.takes_hints() {
/* FP:linker.rs-0392 */             return;
/* FP:linker.rs-0393 */         }
/* FP:linker.rs-0394 */         if self.hinted_static != Some(true) {
/* FP:linker.rs-0395 */             self.link_arg("-Bstatic");
/* FP:linker.rs-0396 */             self.hinted_static = Some(true);
/* FP:linker.rs-0397 */         }
/* FP:linker.rs-0398 */     }
/* FP:linker.rs-0399 */ 
/* FP:linker.rs-0400 */     fn hint_dynamic(&mut self) {
/* FP:linker.rs-0401 */         if !self.takes_hints() {
/* FP:linker.rs-0402 */             return;
/* FP:linker.rs-0403 */         }
/* FP:linker.rs-0404 */         if self.hinted_static != Some(false) {
/* FP:linker.rs-0405 */             self.link_arg("-Bdynamic");
/* FP:linker.rs-0406 */             self.hinted_static = Some(false);
/* FP:linker.rs-0407 */         }
/* FP:linker.rs-0408 */     }
/* FP:linker.rs-0409 */ 
/* FP:linker.rs-0410 */     fn push_linker_plugin_lto_args(&mut self, plugin_path: Option<&OsStr>) {
/* FP:linker.rs-0411 */         if let Some(plugin_path) = plugin_path {
/* FP:linker.rs-0412 */             let mut arg = OsString::from("-plugin=");
/* FP:linker.rs-0413 */             arg.push(plugin_path);
/* FP:linker.rs-0414 */             self.link_arg(&arg);
/* FP:linker.rs-0415 */         }
/* FP:linker.rs-0416 */ 
/* FP:linker.rs-0417 */         let opt_level = match self.sess.opts.optimize {
/* FP:linker.rs-0418 */             config::OptLevel::No => "O0",
/* FP:linker.rs-0419 */             config::OptLevel::Less => "O1",
/* FP:linker.rs-0420 */             config::OptLevel::More | config::OptLevel::Size | config::OptLevel::SizeMin => "O2",
/* FP:linker.rs-0421 */             config::OptLevel::Aggressive => "O3",
/* FP:linker.rs-0422 */         };
/* FP:linker.rs-0423 */ 
/* FP:linker.rs-0424 */         if let Some(path) = &self.sess.opts.unstable_opts.profile_sample_use {
/* FP:linker.rs-0425 */             self.link_arg(&format!("-plugin-opt=sample-profile={}", path.display()));
/* FP:linker.rs-0426 */         };
/* FP:linker.rs-0427 */         self.link_args(&[
/* FP:linker.rs-0428 */             &format!("-plugin-opt={opt_level}"),
/* FP:linker.rs-0429 */             &format!("-plugin-opt=mcpu={}", self.target_cpu),
/* FP:linker.rs-0430 */         ]);
/* FP:linker.rs-0431 */     }
/* FP:linker.rs-0432 */ 
/* FP:linker.rs-0433 */     fn build_dylib(&mut self, crate_type: CrateType, out_filename: &Path) {
/* FP:linker.rs-0434 */         // On mac we need to tell the linker to let this library be rpathed
/* FP:linker.rs-0435 */         if self.sess.target.is_like_darwin {
/* FP:linker.rs-0436 */             if self.is_cc() {
/* FP:linker.rs-0437 */                 // `-dynamiclib` makes `cc` pass `-dylib` to the linker.
/* FP:linker.rs-0438 */                 self.cc_arg("-dynamiclib");
/* FP:linker.rs-0439 */             } else {
/* FP:linker.rs-0440 */                 self.link_arg("-dylib");
/* FP:linker.rs-0441 */                 // Clang also sets `-dynamic`, but that's implied by `-dylib`, so unnecessary.
/* FP:linker.rs-0442 */             }
/* FP:linker.rs-0443 */ 
/* FP:linker.rs-0444 */             // Note that the `osx_rpath_install_name` option here is a hack
/* FP:linker.rs-0445 */             // purely to support bootstrap right now, we should get a more
/* FP:linker.rs-0446 */             // principled solution at some point to force the compiler to pass
/* FP:linker.rs-0447 */             // the right `-Wl,-install_name` with an `@rpath` in it.
/* FP:linker.rs-0448 */             if self.sess.opts.cg.rpath || self.sess.opts.unstable_opts.osx_rpath_install_name {
/* FP:linker.rs-0449 */                 let mut rpath = OsString::from("@rpath/");
/* FP:linker.rs-0450 */                 rpath.push(out_filename.file_name().unwrap());
/* FP:linker.rs-0451 */                 self.link_arg("-install_name").link_arg(rpath);
/* FP:linker.rs-0452 */             }
/* FP:linker.rs-0453 */         } else {
/* FP:linker.rs-0454 */             self.link_or_cc_arg("-shared");
/* FP:linker.rs-0455 */             if let Some(name) = out_filename.file_name() {
/* FP:linker.rs-0456 */                 if self.sess.target.is_like_windows {
/* FP:linker.rs-0457 */                     // The output filename already contains `dll_suffix` so
/* FP:linker.rs-0458 */                     // the resulting import library will have a name in the
/* FP:linker.rs-0459 */                     // form of libfoo.dll.a
/* FP:linker.rs-0460 */                     let (prefix, suffix) = self.sess.staticlib_components(false);
/* FP:linker.rs-0461 */                     let mut implib_name = OsString::from(prefix);
/* FP:linker.rs-0462 */                     implib_name.push(name);
/* FP:linker.rs-0463 */                     implib_name.push(suffix);
/* FP:linker.rs-0464 */                     let mut out_implib = OsString::from("--out-implib=");
/* FP:linker.rs-0465 */                     out_implib.push(out_filename.with_file_name(implib_name));
/* FP:linker.rs-0466 */                     self.link_arg(out_implib);
/* FP:linker.rs-0467 */                 } else if crate_type == CrateType::Dylib {
/* FP:linker.rs-0468 */                     // When dylibs are linked by a full path this value will get into `DT_NEEDED`
/* FP:linker.rs-0469 */                     // instead of the full path, so the library can be later found in some other
/* FP:linker.rs-0470 */                     // location than that specific path.
/* FP:linker.rs-0471 */                     let mut soname = OsString::from("-soname=");
/* FP:linker.rs-0472 */                     soname.push(name);
/* FP:linker.rs-0473 */                     self.link_arg(soname);
/* FP:linker.rs-0474 */                 }
/* FP:linker.rs-0475 */             }
/* FP:linker.rs-0476 */         }
/* FP:linker.rs-0477 */     }
/* FP:linker.rs-0478 */ 
/* FP:linker.rs-0479 */     fn with_as_needed(&mut self, as_needed: bool, f: impl FnOnce(&mut Self)) {
/* FP:linker.rs-0480 */         if !as_needed {
/* FP:linker.rs-0481 */             if self.sess.target.is_like_darwin {
/* FP:linker.rs-0482 */                 // FIXME(81490): ld64 doesn't support these flags but macOS 11
/* FP:linker.rs-0483 */                 // has -needed-l{} / -needed_library {}
/* FP:linker.rs-0484 */                 // but we have no way to detect that here.
/* FP:linker.rs-0485 */                 self.sess.dcx().emit_warn(errors::Ld64UnimplementedModifier);
/* FP:linker.rs-0486 */             } else if self.is_gnu && !self.sess.target.is_like_windows {
/* FP:linker.rs-0487 */                 self.link_arg("--no-as-needed");
/* FP:linker.rs-0488 */             } else {
/* FP:linker.rs-0489 */                 self.sess.dcx().emit_warn(errors::LinkerUnsupportedModifier);
/* FP:linker.rs-0490 */             }
/* FP:linker.rs-0491 */         }
/* FP:linker.rs-0492 */ 
/* FP:linker.rs-0493 */         f(self);
/* FP:linker.rs-0494 */ 
/* FP:linker.rs-0495 */         if !as_needed {
/* FP:linker.rs-0496 */             if self.sess.target.is_like_darwin {
/* FP:linker.rs-0497 */                 // See above FIXME comment
/* FP:linker.rs-0498 */             } else if self.is_gnu && !self.sess.target.is_like_windows {
/* FP:linker.rs-0499 */                 self.link_arg("--as-needed");
/* FP:linker.rs-0500 */             }
/* FP:linker.rs-0501 */         }
/* FP:linker.rs-0502 */     }
/* FP:linker.rs-0503 */ }
/* FP:linker.rs-0504 */ 
/* FP:linker.rs-0505 */ impl<'a> Linker for GccLinker<'a> {
/* FP:linker.rs-0506 */     fn cmd(&mut self) -> &mut Command {
/* FP:linker.rs-0507 */         &mut self.cmd
/* FP:linker.rs-0508 */     }
/* FP:linker.rs-0509 */ 
/* FP:linker.rs-0510 */     fn is_cc(&self) -> bool {
/* FP:linker.rs-0511 */         !self.is_ld
/* FP:linker.rs-0512 */     }
/* FP:linker.rs-0513 */ 
/* FP:linker.rs-0514 */     fn set_output_kind(
/* FP:linker.rs-0515 */         &mut self,
/* FP:linker.rs-0516 */         output_kind: LinkOutputKind,
/* FP:linker.rs-0517 */         crate_type: CrateType,
/* FP:linker.rs-0518 */         out_filename: &Path,
/* FP:linker.rs-0519 */     ) {
/* FP:linker.rs-0520 */         match output_kind {
/* FP:linker.rs-0521 */             LinkOutputKind::DynamicNoPicExe => {
/* FP:linker.rs-0522 */                 if !self.is_ld && self.is_gnu {
/* FP:linker.rs-0523 */                     self.cc_arg("-no-pie");
/* FP:linker.rs-0524 */                 }
/* FP:linker.rs-0525 */             }
/* FP:linker.rs-0526 */             LinkOutputKind::DynamicPicExe => {
/* FP:linker.rs-0527 */                 // noop on windows w/ gcc & ld, error w/ lld
/* FP:linker.rs-0528 */                 if !self.sess.target.is_like_windows {
/* FP:linker.rs-0529 */                     // `-pie` works for both gcc wrapper and ld.
/* FP:linker.rs-0530 */                     self.link_or_cc_arg("-pie");
/* FP:linker.rs-0531 */                 }
/* FP:linker.rs-0532 */             }
/* FP:linker.rs-0533 */             LinkOutputKind::StaticNoPicExe => {
/* FP:linker.rs-0534 */                 // `-static` works for both gcc wrapper and ld.
/* FP:linker.rs-0535 */                 self.link_or_cc_arg("-static");
/* FP:linker.rs-0536 */                 if !self.is_ld && self.is_gnu {
/* FP:linker.rs-0537 */                     self.cc_arg("-no-pie");
/* FP:linker.rs-0538 */                 }
/* FP:linker.rs-0539 */             }
/* FP:linker.rs-0540 */             LinkOutputKind::StaticPicExe => {
/* FP:linker.rs-0541 */                 if !self.is_ld {
/* FP:linker.rs-0542 */                     // Note that combination `-static -pie` doesn't work as expected
/* FP:linker.rs-0543 */                     // for the gcc wrapper, `-static` in that case suppresses `-pie`.
/* FP:linker.rs-0544 */                     self.cc_arg("-static-pie");
/* FP:linker.rs-0545 */                 } else {
/* FP:linker.rs-0546 */                     // `--no-dynamic-linker` and `-z text` are not strictly necessary for producing
/* FP:linker.rs-0547 */                     // a static pie, but currently passed because gcc and clang pass them.
/* FP:linker.rs-0548 */                     // The former suppresses the `INTERP` ELF header specifying dynamic linker,
/* FP:linker.rs-0549 */                     // which is otherwise implicitly injected by ld (but not lld).
/* FP:linker.rs-0550 */                     // The latter doesn't change anything, only ensures that everything is pic.
/* FP:linker.rs-0551 */                     self.link_args(&["-static", "-pie", "--no-dynamic-linker", "-z", "text"]);
/* FP:linker.rs-0552 */                 }
/* FP:linker.rs-0553 */             }
/* FP:linker.rs-0554 */             LinkOutputKind::DynamicDylib => self.build_dylib(crate_type, out_filename),
/* FP:linker.rs-0555 */             LinkOutputKind::StaticDylib => {
/* FP:linker.rs-0556 */                 self.link_or_cc_arg("-static");
/* FP:linker.rs-0557 */                 self.build_dylib(crate_type, out_filename);
/* FP:linker.rs-0558 */             }
/* FP:linker.rs-0559 */             LinkOutputKind::WasiReactorExe => {
/* FP:linker.rs-0560 */                 self.link_args(&["--entry", "_initialize"]);
/* FP:linker.rs-0561 */             }
/* FP:linker.rs-0562 */         }
/* FP:linker.rs-0563 */ 
/* FP:linker.rs-0564 */         // VxWorks compiler driver introduced `--static-crt` flag specifically for rustc,
/* FP:linker.rs-0565 */         // it switches linking for libc and similar system libraries to static without using
/* FP:linker.rs-0566 */         // any `#[link]` attributes in the `libc` crate, see #72782 for details.
/* FP:linker.rs-0567 */         // FIXME: Switch to using `#[link]` attributes in the `libc` crate
/* FP:linker.rs-0568 */         // similarly to other targets.
/* FP:linker.rs-0569 */         if self.sess.target.os == "vxworks"
/* FP:linker.rs-0570 */             && matches!(
/* FP:linker.rs-0571 */                 output_kind,
/* FP:linker.rs-0572 */                 LinkOutputKind::StaticNoPicExe
/* FP:linker.rs-0573 */                     | LinkOutputKind::StaticPicExe
/* FP:linker.rs-0574 */                     | LinkOutputKind::StaticDylib
/* FP:linker.rs-0575 */             )
/* FP:linker.rs-0576 */         {
/* FP:linker.rs-0577 */             self.cc_arg("--static-crt");
/* FP:linker.rs-0578 */         }
/* FP:linker.rs-0579 */ 
/* FP:linker.rs-0580 */         // avr-none doesn't have default ISA, users must specify which specific
/* FP:linker.rs-0581 */         // CPU (well, microcontroller) they are targetting using `-Ctarget-cpu`.
/* FP:linker.rs-0582 */         //
/* FP:linker.rs-0583 */         // Currently this makes sense only when using avr-gcc as a linker, since
/* FP:linker.rs-0584 */         // it brings a couple of hand-written important intrinsics from libgcc.
/* FP:linker.rs-0585 */         if self.sess.target.arch == "avr" && !self.uses_lld {
/* FP:linker.rs-0586 */             self.verbatim_arg(format!("-mmcu={}", self.target_cpu));
/* FP:linker.rs-0587 */         }
/* FP:linker.rs-0588 */     }
/* FP:linker.rs-0589 */ 
/* FP:linker.rs-0590 */     fn link_dylib_by_name(&mut self, name: &str, verbatim: bool, as_needed: bool) {
/* FP:linker.rs-0591 */         if self.sess.target.os == "illumos" && name == "c" {
/* FP:linker.rs-0592 */             // libc will be added via late_link_args on illumos so that it will
/* FP:linker.rs-0593 */             // appear last in the library search order.
/* FP:linker.rs-0594 */             // FIXME: This should be replaced by a more complete and generic
/* FP:linker.rs-0595 */             // mechanism for controlling the order of library arguments passed
/* FP:linker.rs-0596 */             // to the linker.
/* FP:linker.rs-0597 */             return;
/* FP:linker.rs-0598 */         }
/* FP:linker.rs-0599 */         self.hint_dynamic();
/* FP:linker.rs-0600 */         self.with_as_needed(as_needed, |this| {
/* FP:linker.rs-0601 */             let colon = if verbatim && this.is_gnu { ":" } else { "" };
/* FP:linker.rs-0602 */             this.link_or_cc_arg(format!("-l{colon}{name}"));
/* FP:linker.rs-0603 */         });
/* FP:linker.rs-0604 */     }
/* FP:linker.rs-0605 */ 
/* FP:linker.rs-0606 */     fn link_dylib_by_path(&mut self, path: &Path, as_needed: bool) {
/* FP:linker.rs-0607 */         self.hint_dynamic();
/* FP:linker.rs-0608 */         self.with_as_needed(as_needed, |this| {
/* FP:linker.rs-0609 */             this.link_or_cc_arg(path);
/* FP:linker.rs-0610 */         })
/* FP:linker.rs-0611 */     }
/* FP:linker.rs-0612 */ 
/* FP:linker.rs-0613 */     fn link_framework_by_name(&mut self, name: &str, _verbatim: bool, as_needed: bool) {
/* FP:linker.rs-0614 */         self.hint_dynamic();
/* FP:linker.rs-0615 */         if !as_needed {
/* FP:linker.rs-0616 */             // FIXME(81490): ld64 as of macOS 11 supports the -needed_framework
/* FP:linker.rs-0617 */             // flag but we have no way to detect that here.
/* FP:linker.rs-0618 */             // self.link_or_cc_arg("-needed_framework").link_or_cc_arg(name);
/* FP:linker.rs-0619 */             self.sess.dcx().emit_warn(errors::Ld64UnimplementedModifier);
/* FP:linker.rs-0620 */         }
/* FP:linker.rs-0621 */         self.link_or_cc_args(&["-framework", name]);
/* FP:linker.rs-0622 */     }
/* FP:linker.rs-0623 */ 
/* FP:linker.rs-0624 */     fn link_staticlib_by_name(&mut self, name: &str, verbatim: bool, whole_archive: bool) {
/* FP:linker.rs-0625 */         self.hint_static();
/* FP:linker.rs-0626 */         let colon = if verbatim && self.is_gnu { ":" } else { "" };
/* FP:linker.rs-0627 */         if !whole_archive {
/* FP:linker.rs-0628 */             self.link_or_cc_arg(format!("-l{colon}{name}"));
/* FP:linker.rs-0629 */         } else if self.sess.target.is_like_darwin {
/* FP:linker.rs-0630 */             // -force_load is the macOS equivalent of --whole-archive, but it
/* FP:linker.rs-0631 */             // involves passing the full path to the library to link.
/* FP:linker.rs-0632 */             self.link_arg("-force_load");
/* FP:linker.rs-0633 */             self.link_arg(find_native_static_library(name, verbatim, self.sess));
/* FP:linker.rs-0634 */         } else {
/* FP:linker.rs-0635 */             self.link_arg("--whole-archive")
/* FP:linker.rs-0636 */                 .link_or_cc_arg(format!("-l{colon}{name}"))
/* FP:linker.rs-0637 */                 .link_arg("--no-whole-archive");
/* FP:linker.rs-0638 */         }
/* FP:linker.rs-0639 */     }
/* FP:linker.rs-0640 */ 
/* FP:linker.rs-0641 */     fn link_staticlib_by_path(&mut self, path: &Path, whole_archive: bool) {
/* FP:linker.rs-0642 */         self.hint_static();
/* FP:linker.rs-0643 */         if !whole_archive {
/* FP:linker.rs-0644 */             self.link_or_cc_arg(path);
/* FP:linker.rs-0645 */         } else if self.sess.target.is_like_darwin {
/* FP:linker.rs-0646 */             self.link_arg("-force_load").link_arg(path);
/* FP:linker.rs-0647 */         } else {
/* FP:linker.rs-0648 */             self.link_arg("--whole-archive").link_arg(path).link_arg("--no-whole-archive");
/* FP:linker.rs-0649 */         }
/* FP:linker.rs-0650 */     }
/* FP:linker.rs-0651 */ 
/* FP:linker.rs-0652 */     fn framework_path(&mut self, path: &Path) {
/* FP:linker.rs-0653 */         self.link_or_cc_arg("-F").link_or_cc_arg(path);
/* FP:linker.rs-0654 */     }
/* FP:linker.rs-0655 */     fn full_relro(&mut self) {
/* FP:linker.rs-0656 */         self.link_args(&["-z", "relro", "-z", "now"]);
/* FP:linker.rs-0657 */     }
/* FP:linker.rs-0658 */     fn partial_relro(&mut self) {
/* FP:linker.rs-0659 */         self.link_args(&["-z", "relro"]);
/* FP:linker.rs-0660 */     }
/* FP:linker.rs-0661 */     fn no_relro(&mut self) {
/* FP:linker.rs-0662 */         self.link_args(&["-z", "norelro"]);
/* FP:linker.rs-0663 */     }
/* FP:linker.rs-0664 */ 
/* FP:linker.rs-0665 */     fn gc_sections(&mut self, keep_metadata: bool) {
/* FP:linker.rs-0666 */         // The dead_strip option to the linker specifies that functions and data
/* FP:linker.rs-0667 */         // unreachable by the entry point will be removed. This is quite useful
/* FP:linker.rs-0668 */         // with Rust's compilation model of compiling libraries at a time into
/* FP:linker.rs-0669 */         // one object file. For example, this brings hello world from 1.7MB to
/* FP:linker.rs-0670 */         // 458K.
/* FP:linker.rs-0671 */         //
/* FP:linker.rs-0672 */         // Note that this is done for both executables and dynamic libraries. We
/* FP:linker.rs-0673 */         // won't get much benefit from dylibs because LLVM will have already
/* FP:linker.rs-0674 */         // stripped away as much as it could. This has not been seen to impact
/* FP:linker.rs-0675 */         // link times negatively.
/* FP:linker.rs-0676 */         //
/* FP:linker.rs-0677 */         // -dead_strip can't be part of the pre_link_args because it's also used
/* FP:linker.rs-0678 */         // for partial linking when using multiple codegen units (-r). So we
/* FP:linker.rs-0679 */         // insert it here.
/* FP:linker.rs-0680 */         if self.sess.target.is_like_darwin {
/* FP:linker.rs-0681 */             self.link_arg("-dead_strip");
/* FP:linker.rs-0682 */ 
/* FP:linker.rs-0683 */         // If we're building a dylib, we don't use --gc-sections because LLVM
/* FP:linker.rs-0684 */         // has already done the best it can do, and we also don't want to
/* FP:linker.rs-0685 */         // eliminate the metadata. If we're building an executable, however,
/* FP:linker.rs-0686 */         // --gc-sections drops the size of hello world from 1.8MB to 597K, a 67%
/* FP:linker.rs-0687 */         // reduction.
/* FP:linker.rs-0688 */         } else if (self.is_gnu || self.sess.target.is_like_wasm) && !keep_metadata {
/* FP:linker.rs-0689 */             self.link_arg("--gc-sections");
/* FP:linker.rs-0690 */         }
/* FP:linker.rs-0691 */     }
/* FP:linker.rs-0692 */ 
/* FP:linker.rs-0693 */     fn optimize(&mut self) {
/* FP:linker.rs-0694 */         if !self.is_gnu && !self.sess.target.is_like_wasm {
/* FP:linker.rs-0695 */             return;
/* FP:linker.rs-0696 */         }
/* FP:linker.rs-0697 */ 
/* FP:linker.rs-0698 */         // GNU-style linkers support optimization with -O. GNU ld doesn't
/* FP:linker.rs-0699 */         // need a numeric argument, but other linkers do.
/* FP:linker.rs-0700 */         if self.sess.opts.optimize == config::OptLevel::More
/* FP:linker.rs-0701 */             || self.sess.opts.optimize == config::OptLevel::Aggressive
/* FP:linker.rs-0702 */         {
/* FP:linker.rs-0703 */             self.link_arg("-O1");
/* FP:linker.rs-0704 */         }
/* FP:linker.rs-0705 */     }
/* FP:linker.rs-0706 */ 
/* FP:linker.rs-0707 */     fn pgo_gen(&mut self) {
/* FP:linker.rs-0708 */         if !self.is_gnu {
/* FP:linker.rs-0709 */             return;
/* FP:linker.rs-0710 */         }
/* FP:linker.rs-0711 */ 
/* FP:linker.rs-0712 */         // If we're doing PGO generation stuff and on a GNU-like linker, use the
/* FP:linker.rs-0713 */         // "-u" flag to properly pull in the profiler runtime bits.
/* FP:linker.rs-0714 */         //
/* FP:linker.rs-0715 */         // This is because LLVM otherwise won't add the needed initialization
/* FP:linker.rs-0716 */         // for us on Linux (though the extra flag should be harmless if it
/* FP:linker.rs-0717 */         // does).
/* FP:linker.rs-0718 */         //
/* FP:linker.rs-0719 */         // See https://reviews.llvm.org/D14033 and https://reviews.llvm.org/D14030.
/* FP:linker.rs-0720 */         //
/* FP:linker.rs-0721 */         // Though it may be worth to try to revert those changes upstream, since
/* FP:linker.rs-0722 */         // the overhead of the initialization should be minor.
/* FP:linker.rs-0723 */         self.link_or_cc_args(&["-u", "__llvm_profile_runtime"]);
/* FP:linker.rs-0724 */     }
/* FP:linker.rs-0725 */ 
/* FP:linker.rs-0726 */     fn control_flow_guard(&mut self) {}
/* FP:linker.rs-0727 */ 
/* FP:linker.rs-0728 */     fn ehcont_guard(&mut self) {}
/* FP:linker.rs-0729 */ 
/* FP:linker.rs-0730 */     fn debuginfo(&mut self, strip: Strip, _: &[PathBuf]) {
/* FP:linker.rs-0731 */         // MacOS linker doesn't support stripping symbols directly anymore.
/* FP:linker.rs-0732 */         if self.sess.target.is_like_darwin {
/* FP:linker.rs-0733 */             return;
/* FP:linker.rs-0734 */         }
/* FP:linker.rs-0735 */ 
/* FP:linker.rs-0736 */         match strip {
/* FP:linker.rs-0737 */             Strip::None => {}
/* FP:linker.rs-0738 */             Strip::Debuginfo => {
/* FP:linker.rs-0739 */                 // The illumos linker does not support --strip-debug although
/* FP:linker.rs-0740 */                 // it does support --strip-all as a compatibility alias for -s.
/* FP:linker.rs-0741 */                 // The --strip-debug case is handled by running an external
/* FP:linker.rs-0742 */                 // `strip` utility as a separate step after linking.
/* FP:linker.rs-0743 */                 if !self.sess.target.is_like_solaris {
/* FP:linker.rs-0744 */                     self.link_arg("--strip-debug");
/* FP:linker.rs-0745 */                 }
/* FP:linker.rs-0746 */             }
/* FP:linker.rs-0747 */             Strip::Symbols => {
/* FP:linker.rs-0748 */                 self.link_arg("--strip-all");
/* FP:linker.rs-0749 */             }
/* FP:linker.rs-0750 */         }
/* FP:linker.rs-0751 */         match self.sess.opts.unstable_opts.debuginfo_compression {
/* FP:linker.rs-0752 */             config::DebugInfoCompression::None => {}
/* FP:linker.rs-0753 */             config::DebugInfoCompression::Zlib => {
/* FP:linker.rs-0754 */                 self.link_arg("--compress-debug-sections=zlib");
/* FP:linker.rs-0755 */             }
/* FP:linker.rs-0756 */             config::DebugInfoCompression::Zstd => {
/* FP:linker.rs-0757 */                 self.link_arg("--compress-debug-sections=zstd");
/* FP:linker.rs-0758 */             }
/* FP:linker.rs-0759 */         }
/* FP:linker.rs-0760 */     }
/* FP:linker.rs-0761 */ 
/* FP:linker.rs-0762 */     fn no_crt_objects(&mut self) {
/* FP:linker.rs-0763 */         if !self.is_ld {
/* FP:linker.rs-0764 */             self.cc_arg("-nostartfiles");
/* FP:linker.rs-0765 */         }
/* FP:linker.rs-0766 */     }
/* FP:linker.rs-0767 */ 
/* FP:linker.rs-0768 */     fn no_default_libraries(&mut self) {
/* FP:linker.rs-0769 */         if !self.is_ld {
/* FP:linker.rs-0770 */             self.cc_arg("-nodefaultlibs");
/* FP:linker.rs-0771 */         }
/* FP:linker.rs-0772 */     }
/* FP:linker.rs-0773 */ 
/* FP:linker.rs-0774 */     fn export_symbols(
/* FP:linker.rs-0775 */         &mut self,
/* FP:linker.rs-0776 */         tmpdir: &Path,
/* FP:linker.rs-0777 */         crate_type: CrateType,
/* FP:linker.rs-0778 */         symbols: &[(String, SymbolExportKind)],
/* FP:linker.rs-0779 */     ) {
/* FP:linker.rs-0780 */         // Symbol visibility in object files typically takes care of this.
/* FP:linker.rs-0781 */         if crate_type == CrateType::Executable {
/* FP:linker.rs-0782 */             let should_export_executable_symbols =
/* FP:linker.rs-0783 */                 self.sess.opts.unstable_opts.export_executable_symbols;
/* FP:linker.rs-0784 */             if self.sess.target.override_export_symbols.is_none()
/* FP:linker.rs-0785 */                 && !should_export_executable_symbols
/* FP:linker.rs-0786 */             {
/* FP:linker.rs-0787 */                 return;
/* FP:linker.rs-0788 */             }
/* FP:linker.rs-0789 */         }
/* FP:linker.rs-0790 */ 
/* FP:linker.rs-0791 */         // We manually create a list of exported symbols to ensure we don't expose any more.
/* FP:linker.rs-0792 */         // The object files have far more public symbols than we actually want to export,
/* FP:linker.rs-0793 */         // so we hide them all here.
/* FP:linker.rs-0794 */ 
/* FP:linker.rs-0795 */         if !self.sess.target.limit_rdylib_exports {
/* FP:linker.rs-0796 */             return;
/* FP:linker.rs-0797 */         }
/* FP:linker.rs-0798 */ 
/* FP:linker.rs-0799 */         let path = tmpdir.join(if self.sess.target.is_like_windows { "list.def" } else { "list" });
/* FP:linker.rs-0800 */         debug!("EXPORTED SYMBOLS:");
/* FP:linker.rs-0801 */ 
/* FP:linker.rs-0802 */         if self.sess.target.is_like_darwin {
/* FP:linker.rs-0803 */             // Write a plain, newline-separated list of symbols
/* FP:linker.rs-0804 */             let res: io::Result<()> = try {
/* FP:linker.rs-0805 */                 let mut f = File::create_buffered(&path)?;
/* FP:linker.rs-0806 */                 for (sym, _) in symbols {
/* FP:linker.rs-0807 */                     debug!("  _{sym}");
/* FP:linker.rs-0808 */                     writeln!(f, "_{sym}")?;
/* FP:linker.rs-0809 */                 }
/* FP:linker.rs-0810 */             };
/* FP:linker.rs-0811 */             if let Err(error) = res {
/* FP:linker.rs-0812 */                 self.sess.dcx().emit_fatal(errors::LibDefWriteFailure { error });
/* FP:linker.rs-0813 */             }
/* FP:linker.rs-0814 */             self.link_arg("-exported_symbols_list").link_arg(path);
/* FP:linker.rs-0815 */         } else if self.sess.target.is_like_windows {
/* FP:linker.rs-0816 */             let res: io::Result<()> = try {
/* FP:linker.rs-0817 */                 let mut f = File::create_buffered(&path)?;
/* FP:linker.rs-0818 */ 
/* FP:linker.rs-0819 */                 // .def file similar to MSVC one but without LIBRARY section
/* FP:linker.rs-0820 */                 // because LD doesn't like when it's empty
/* FP:linker.rs-0821 */                 writeln!(f, "EXPORTS")?;
/* FP:linker.rs-0822 */                 for (symbol, kind) in symbols {
/* FP:linker.rs-0823 */                     let kind_marker = if *kind == SymbolExportKind::Data { " DATA" } else { "" };
/* FP:linker.rs-0824 */                     debug!("  _{symbol}");
/* FP:linker.rs-0825 */                     // Quote the name in case it's reserved by linker in some way
/* FP:linker.rs-0826 */                     // (this accounts for names with dots in particular).
/* FP:linker.rs-0827 */                     writeln!(f, "  \"{symbol}\"{kind_marker}")?;
/* FP:linker.rs-0828 */                 }
/* FP:linker.rs-0829 */             };
/* FP:linker.rs-0830 */             if let Err(error) = res {
/* FP:linker.rs-0831 */                 self.sess.dcx().emit_fatal(errors::LibDefWriteFailure { error });
/* FP:linker.rs-0832 */             }
/* FP:linker.rs-0833 */             self.link_arg(path);
/* FP:linker.rs-0834 */         } else if crate_type == CrateType::Executable && !self.sess.target.is_like_solaris {
/* FP:linker.rs-0835 */             let res: io::Result<()> = try {
/* FP:linker.rs-0836 */                 let mut f = File::create_buffered(&path)?;
/* FP:linker.rs-0837 */                 writeln!(f, "{{")?;
/* FP:linker.rs-0838 */                 for (sym, _) in symbols {
/* FP:linker.rs-0839 */                     debug!(sym);
/* FP:linker.rs-0840 */                     writeln!(f, "  {sym};")?;
/* FP:linker.rs-0841 */                 }
/* FP:linker.rs-0842 */                 writeln!(f, "}};")?;
/* FP:linker.rs-0843 */             };
/* FP:linker.rs-0844 */             if let Err(error) = res {
/* FP:linker.rs-0845 */                 self.sess.dcx().emit_fatal(errors::VersionScriptWriteFailure { error });
/* FP:linker.rs-0846 */             }
/* FP:linker.rs-0847 */             self.link_arg("--dynamic-list").link_arg(path);
/* FP:linker.rs-0848 */         } else {
/* FP:linker.rs-0849 */             // Write an LD version script
/* FP:linker.rs-0850 */             let res: io::Result<()> = try {
/* FP:linker.rs-0851 */                 let mut f = File::create_buffered(&path)?;
/* FP:linker.rs-0852 */                 writeln!(f, "{{")?;
/* FP:linker.rs-0853 */                 if !symbols.is_empty() {
/* FP:linker.rs-0854 */                     writeln!(f, "  global:")?;
/* FP:linker.rs-0855 */                     for (sym, _) in symbols {
/* FP:linker.rs-0856 */                         debug!("    {sym};");
/* FP:linker.rs-0857 */                         writeln!(f, "    {sym};")?;
/* FP:linker.rs-0858 */                     }
/* FP:linker.rs-0859 */                 }
/* FP:linker.rs-0860 */                 writeln!(f, "\n  local:\n    *;\n}};")?;
/* FP:linker.rs-0861 */             };
/* FP:linker.rs-0862 */             if let Err(error) = res {
/* FP:linker.rs-0863 */                 self.sess.dcx().emit_fatal(errors::VersionScriptWriteFailure { error });
/* FP:linker.rs-0864 */             }
/* FP:linker.rs-0865 */             if self.sess.target.is_like_solaris {
/* FP:linker.rs-0866 */                 self.link_arg("-M").link_arg(path);
/* FP:linker.rs-0867 */             } else {
/* FP:linker.rs-0868 */                 let mut arg = OsString::from("--version-script=");
/* FP:linker.rs-0869 */                 arg.push(path);
/* FP:linker.rs-0870 */                 self.link_arg(arg).link_arg("--no-undefined-version");
/* FP:linker.rs-0871 */             }
/* FP:linker.rs-0872 */         }
/* FP:linker.rs-0873 */     }
/* FP:linker.rs-0874 */ 
/* FP:linker.rs-0875 */     fn subsystem(&mut self, subsystem: &str) {
/* FP:linker.rs-0876 */         self.link_args(&["--subsystem", subsystem]);
/* FP:linker.rs-0877 */     }
/* FP:linker.rs-0878 */ 
/* FP:linker.rs-0879 */     fn reset_per_library_state(&mut self) {
/* FP:linker.rs-0880 */         self.hint_dynamic(); // Reset to default before returning the composed command line.
/* FP:linker.rs-0881 */     }
/* FP:linker.rs-0882 */ 
/* FP:linker.rs-0883 */     fn linker_plugin_lto(&mut self) {
/* FP:linker.rs-0884 */         match self.sess.opts.cg.linker_plugin_lto {
/* FP:linker.rs-0885 */             LinkerPluginLto::Disabled => {
/* FP:linker.rs-0886 */                 // Nothing to do
/* FP:linker.rs-0887 */             }
/* FP:linker.rs-0888 */             LinkerPluginLto::LinkerPluginAuto => {
/* FP:linker.rs-0889 */                 self.push_linker_plugin_lto_args(None);
/* FP:linker.rs-0890 */             }
/* FP:linker.rs-0891 */             LinkerPluginLto::LinkerPlugin(ref path) => {
/* FP:linker.rs-0892 */                 self.push_linker_plugin_lto_args(Some(path.as_os_str()));
/* FP:linker.rs-0893 */             }
/* FP:linker.rs-0894 */         }
/* FP:linker.rs-0895 */     }
/* FP:linker.rs-0896 */ 
/* FP:linker.rs-0897 */     // Add the `GNU_EH_FRAME` program header which is required to locate unwinding information.
/* FP:linker.rs-0898 */     // Some versions of `gcc` add it implicitly, some (e.g. `musl-gcc`) don't,
/* FP:linker.rs-0899 */     // so we just always add it.
/* FP:linker.rs-0900 */     fn add_eh_frame_header(&mut self) {
/* FP:linker.rs-0901 */         self.link_arg("--eh-frame-hdr");
/* FP:linker.rs-0902 */     }
/* FP:linker.rs-0903 */ 
/* FP:linker.rs-0904 */     fn add_no_exec(&mut self) {
/* FP:linker.rs-0905 */         if self.sess.target.is_like_windows {
/* FP:linker.rs-0906 */             self.link_arg("--nxcompat");
/* FP:linker.rs-0907 */         } else if self.is_gnu {
/* FP:linker.rs-0908 */             self.link_args(&["-z", "noexecstack"]);
/* FP:linker.rs-0909 */         }
/* FP:linker.rs-0910 */     }
/* FP:linker.rs-0911 */ 
/* FP:linker.rs-0912 */     fn add_as_needed(&mut self) {
/* FP:linker.rs-0913 */         if self.is_gnu && !self.sess.target.is_like_windows {
/* FP:linker.rs-0914 */             self.link_arg("--as-needed");
/* FP:linker.rs-0915 */         } else if self.sess.target.is_like_solaris {
/* FP:linker.rs-0916 */             // -z ignore is the Solaris equivalent to the GNU ld --as-needed option
/* FP:linker.rs-0917 */             self.link_args(&["-z", "ignore"]);
/* FP:linker.rs-0918 */         }
/* FP:linker.rs-0919 */     }
/* FP:linker.rs-0920 */ }
/* FP:linker.rs-0921 */ 
/* FP:linker.rs-0922 */ struct MsvcLinker<'a> {
/* FP:linker.rs-0923 */     cmd: Command,
/* FP:linker.rs-0924 */     sess: &'a Session,
/* FP:linker.rs-0925 */ }
/* FP:linker.rs-0926 */ 
/* FP:linker.rs-0927 */ impl<'a> Linker for MsvcLinker<'a> {
/* FP:linker.rs-0928 */     fn cmd(&mut self) -> &mut Command {
/* FP:linker.rs-0929 */         &mut self.cmd
/* FP:linker.rs-0930 */     }
/* FP:linker.rs-0931 */ 
/* FP:linker.rs-0932 */     fn set_output_kind(
/* FP:linker.rs-0933 */         &mut self,
/* FP:linker.rs-0934 */         output_kind: LinkOutputKind,
/* FP:linker.rs-0935 */         _crate_type: CrateType,
/* FP:linker.rs-0936 */         out_filename: &Path,
/* FP:linker.rs-0937 */     ) {
/* FP:linker.rs-0938 */         match output_kind {
/* FP:linker.rs-0939 */             LinkOutputKind::DynamicNoPicExe
/* FP:linker.rs-0940 */             | LinkOutputKind::DynamicPicExe
/* FP:linker.rs-0941 */             | LinkOutputKind::StaticNoPicExe
/* FP:linker.rs-0942 */             | LinkOutputKind::StaticPicExe => {}
/* FP:linker.rs-0943 */             LinkOutputKind::DynamicDylib | LinkOutputKind::StaticDylib => {
/* FP:linker.rs-0944 */                 self.link_arg("/DLL");
/* FP:linker.rs-0945 */                 let mut arg: OsString = "/IMPLIB:".into();
/* FP:linker.rs-0946 */                 arg.push(out_filename.with_extension("dll.lib"));
/* FP:linker.rs-0947 */                 self.link_arg(arg);
/* FP:linker.rs-0948 */             }
/* FP:linker.rs-0949 */             LinkOutputKind::WasiReactorExe => {
/* FP:linker.rs-0950 */                 panic!("can't link as reactor on non-wasi target");
/* FP:linker.rs-0951 */             }
/* FP:linker.rs-0952 */         }
/* FP:linker.rs-0953 */     }
/* FP:linker.rs-0954 */ 
/* FP:linker.rs-0955 */     fn link_dylib_by_name(&mut self, name: &str, verbatim: bool, _as_needed: bool) {
/* FP:linker.rs-0956 */         // On MSVC-like targets rustc supports import libraries using alternative naming
/* FP:linker.rs-0957 */         // scheme (`libfoo.a`) unsupported by linker, search for such libraries manually.
/* FP:linker.rs-0958 */         if let Some(path) = try_find_native_dynamic_library(self.sess, name, verbatim) {
/* FP:linker.rs-0959 */             self.link_arg(path);
/* FP:linker.rs-0960 */         } else {
/* FP:linker.rs-0961 */             self.link_arg(format!("{}{}", name, if verbatim { "" } else { ".lib" }));
/* FP:linker.rs-0962 */         }
/* FP:linker.rs-0963 */     }
/* FP:linker.rs-0964 */ 
/* FP:linker.rs-0965 */     fn link_dylib_by_path(&mut self, path: &Path, _as_needed: bool) {
/* FP:linker.rs-0966 */         // When producing a dll, MSVC linker may not emit an implib file if the dll doesn't export
/* FP:linker.rs-0967 */         // any symbols, so we skip linking if the implib file is not present.
/* FP:linker.rs-0968 */         let implib_path = path.with_extension("dll.lib");
/* FP:linker.rs-0969 */         if implib_path.exists() {
/* FP:linker.rs-0970 */             self.link_or_cc_arg(implib_path);
/* FP:linker.rs-0971 */         }
/* FP:linker.rs-0972 */     }
/* FP:linker.rs-0973 */ 
/* FP:linker.rs-0974 */     fn link_staticlib_by_name(&mut self, name: &str, verbatim: bool, whole_archive: bool) {
/* FP:linker.rs-0975 */         // On MSVC-like targets rustc supports static libraries using alternative naming
/* FP:linker.rs-0976 */         // scheme (`libfoo.a`) unsupported by linker, search for such libraries manually.
/* FP:linker.rs-0977 */         if let Some(path) = try_find_native_static_library(self.sess, name, verbatim) {
/* FP:linker.rs-0978 */             self.link_staticlib_by_path(&path, whole_archive);
/* FP:linker.rs-0979 */         } else {
/* FP:linker.rs-0980 */             let opts = if whole_archive { "/WHOLEARCHIVE:" } else { "" };
/* FP:linker.rs-0981 */             let (prefix, suffix) = self.sess.staticlib_components(verbatim);
/* FP:linker.rs-0982 */             self.link_arg(format!("{opts}{prefix}{name}{suffix}"));
/* FP:linker.rs-0983 */         }
/* FP:linker.rs-0984 */     }
/* FP:linker.rs-0985 */ 
/* FP:linker.rs-0986 */     fn link_staticlib_by_path(&mut self, path: &Path, whole_archive: bool) {
/* FP:linker.rs-0987 */         if !whole_archive {
/* FP:linker.rs-0988 */             self.link_arg(path);
/* FP:linker.rs-0989 */         } else {
/* FP:linker.rs-0990 */             let mut arg = OsString::from("/WHOLEARCHIVE:");
/* FP:linker.rs-0991 */             arg.push(path);
/* FP:linker.rs-0992 */             self.link_arg(arg);
/* FP:linker.rs-0993 */         }
/* FP:linker.rs-0994 */     }
/* FP:linker.rs-0995 */ 
/* FP:linker.rs-0996 */     fn gc_sections(&mut self, _keep_metadata: bool) {
/* FP:linker.rs-0997 */         // MSVC's ICF (Identical COMDAT Folding) link optimization is
/* FP:linker.rs-0998 */         // slow for Rust and thus we disable it by default when not in
/* FP:linker.rs-0999 */         // optimization build.
/* FP:linker.rs-1000 */         if self.sess.opts.optimize != config::OptLevel::No {
/* FP:linker.rs-1001 */             self.link_arg("/OPT:REF,ICF");
/* FP:linker.rs-1002 */         } else {
/* FP:linker.rs-1003 */             // It is necessary to specify NOICF here, because /OPT:REF
/* FP:linker.rs-1004 */             // implies ICF by default.
/* FP:linker.rs-1005 */             self.link_arg("/OPT:REF,NOICF");
/* FP:linker.rs-1006 */         }
/* FP:linker.rs-1007 */     }
/* FP:linker.rs-1008 */ 
/* FP:linker.rs-1009 */     fn full_relro(&mut self) {
/* FP:linker.rs-1010 */         // noop
/* FP:linker.rs-1011 */     }
/* FP:linker.rs-1012 */ 
/* FP:linker.rs-1013 */     fn partial_relro(&mut self) {
/* FP:linker.rs-1014 */         // noop
/* FP:linker.rs-1015 */     }
/* FP:linker.rs-1016 */ 
/* FP:linker.rs-1017 */     fn no_relro(&mut self) {
/* FP:linker.rs-1018 */         // noop
/* FP:linker.rs-1019 */     }
/* FP:linker.rs-1020 */ 
/* FP:linker.rs-1021 */     fn no_crt_objects(&mut self) {
/* FP:linker.rs-1022 */         // noop
/* FP:linker.rs-1023 */     }
/* FP:linker.rs-1024 */ 
/* FP:linker.rs-1025 */     fn no_default_libraries(&mut self) {
/* FP:linker.rs-1026 */         self.link_arg("/NODEFAULTLIB");
/* FP:linker.rs-1027 */     }
/* FP:linker.rs-1028 */ 
/* FP:linker.rs-1029 */     fn include_path(&mut self, path: &Path) {
/* FP:linker.rs-1030 */         let mut arg = OsString::from("/LIBPATH:");
/* FP:linker.rs-1031 */         arg.push(path);
/* FP:linker.rs-1032 */         self.link_arg(&arg);
/* FP:linker.rs-1033 */     }
/* FP:linker.rs-1034 */ 
/* FP:linker.rs-1035 */     fn output_filename(&mut self, path: &Path) {
/* FP:linker.rs-1036 */         let mut arg = OsString::from("/OUT:");
/* FP:linker.rs-1037 */         arg.push(path);
/* FP:linker.rs-1038 */         self.link_arg(&arg);
/* FP:linker.rs-1039 */     }
/* FP:linker.rs-1040 */ 
/* FP:linker.rs-1041 */     fn optimize(&mut self) {
/* FP:linker.rs-1042 */         // Needs more investigation of `/OPT` arguments
/* FP:linker.rs-1043 */     }
/* FP:linker.rs-1044 */ 
/* FP:linker.rs-1045 */     fn pgo_gen(&mut self) {
/* FP:linker.rs-1046 */         // Nothing needed here.
/* FP:linker.rs-1047 */     }
/* FP:linker.rs-1048 */ 
/* FP:linker.rs-1049 */     fn control_flow_guard(&mut self) {
/* FP:linker.rs-1050 */         self.link_arg("/guard:cf");
/* FP:linker.rs-1051 */     }
/* FP:linker.rs-1052 */ 
/* FP:linker.rs-1053 */     fn ehcont_guard(&mut self) {
/* FP:linker.rs-1054 */         if self.sess.target.pointer_width == 64 {
/* FP:linker.rs-1055 */             self.link_arg("/guard:ehcont");
/* FP:linker.rs-1056 */         }
/* FP:linker.rs-1057 */     }
/* FP:linker.rs-1058 */ 
/* FP:linker.rs-1059 */     fn debuginfo(&mut self, _strip: Strip, natvis_debugger_visualizers: &[PathBuf]) {
/* FP:linker.rs-1060 */         // This will cause the Microsoft linker to generate a PDB file
/* FP:linker.rs-1061 */         // from the CodeView line tables in the object files.
/* FP:linker.rs-1062 */         self.link_arg("/DEBUG");
/* FP:linker.rs-1063 */ 
/* FP:linker.rs-1064 */         // Default to emitting only the file name of the PDB file into
/* FP:linker.rs-1065 */         // the binary instead of the full path. Emitting the full path
/* FP:linker.rs-1066 */         // may leak private information (such as user names).
/* FP:linker.rs-1067 */         // See https://github.com/rust-lang/rust/issues/87825.
/* FP:linker.rs-1068 */         //
/* FP:linker.rs-1069 */         // This default behavior can be overridden by explicitly passing
/* FP:linker.rs-1070 */         // `-Clink-arg=/PDBALTPATH:...` to rustc.
/* FP:linker.rs-1071 */         self.link_arg("/PDBALTPATH:%_PDB%");
/* FP:linker.rs-1072 */ 
/* FP:linker.rs-1073 */         // This will cause the Microsoft linker to embed .natvis info into the PDB file
/* FP:linker.rs-1074 */         let natvis_dir_path = self.sess.opts.sysroot.path().join("lib\\rustlib\\etc");
/* FP:linker.rs-1075 */         if let Ok(natvis_dir) = fs::read_dir(&natvis_dir_path) {
/* FP:linker.rs-1076 */             for entry in natvis_dir {
/* FP:linker.rs-1077 */                 match entry {
/* FP:linker.rs-1078 */                     Ok(entry) => {
/* FP:linker.rs-1079 */                         let path = entry.path();
/* FP:linker.rs-1080 */                         if path.extension() == Some("natvis".as_ref()) {
/* FP:linker.rs-1081 */                             let mut arg = OsString::from("/NATVIS:");
/* FP:linker.rs-1082 */                             arg.push(path);
/* FP:linker.rs-1083 */                             self.link_arg(arg);
/* FP:linker.rs-1084 */                         }
/* FP:linker.rs-1085 */                     }
/* FP:linker.rs-1086 */                     Err(error) => {
/* FP:linker.rs-1087 */                         self.sess.dcx().emit_warn(errors::NoNatvisDirectory { error });
/* FP:linker.rs-1088 */                     }
/* FP:linker.rs-1089 */                 }
/* FP:linker.rs-1090 */             }
/* FP:linker.rs-1091 */         }
/* FP:linker.rs-1092 */ 
/* FP:linker.rs-1093 */         // This will cause the Microsoft linker to embed .natvis info for all crates into the PDB file
/* FP:linker.rs-1094 */         for path in natvis_debugger_visualizers {
/* FP:linker.rs-1095 */             let mut arg = OsString::from("/NATVIS:");
/* FP:linker.rs-1096 */             arg.push(path);
/* FP:linker.rs-1097 */             self.link_arg(arg);
/* FP:linker.rs-1098 */         }
/* FP:linker.rs-1099 */     }
/* FP:linker.rs-1100 */ 
/* FP:linker.rs-1101 */     // Currently the compiler doesn't use `dllexport` (an LLVM attribute) to
/* FP:linker.rs-1102 */     // export symbols from a dynamic library. When building a dynamic library,
/* FP:linker.rs-1103 */     // however, we're going to want some symbols exported, so this function
/* FP:linker.rs-1104 */     // generates a DEF file which lists all the symbols.
/* FP:linker.rs-1105 */     //
/* FP:linker.rs-1106 */     // The linker will read this `*.def` file and export all the symbols from
/* FP:linker.rs-1107 */     // the dynamic library. Note that this is not as simple as just exporting
/* FP:linker.rs-1108 */     // all the symbols in the current crate (as specified by `codegen.reachable`)
/* FP:linker.rs-1109 */     // but rather we also need to possibly export the symbols of upstream
/* FP:linker.rs-1110 */     // crates. Upstream rlibs may be linked statically to this dynamic library,
/* FP:linker.rs-1111 */     // in which case they may continue to transitively be used and hence need
/* FP:linker.rs-1112 */     // their symbols exported.
/* FP:linker.rs-1113 */     fn export_symbols(
/* FP:linker.rs-1114 */         &mut self,
/* FP:linker.rs-1115 */         tmpdir: &Path,
/* FP:linker.rs-1116 */         crate_type: CrateType,
/* FP:linker.rs-1117 */         symbols: &[(String, SymbolExportKind)],
/* FP:linker.rs-1118 */     ) {
/* FP:linker.rs-1119 */         // Symbol visibility takes care of this typically
/* FP:linker.rs-1120 */         if crate_type == CrateType::Executable {
/* FP:linker.rs-1121 */             let should_export_executable_symbols =
/* FP:linker.rs-1122 */                 self.sess.opts.unstable_opts.export_executable_symbols;
/* FP:linker.rs-1123 */             if !should_export_executable_symbols {
/* FP:linker.rs-1124 */                 return;
/* FP:linker.rs-1125 */             }
/* FP:linker.rs-1126 */         }
/* FP:linker.rs-1127 */ 
/* FP:linker.rs-1128 */         let path = tmpdir.join("lib.def");
/* FP:linker.rs-1129 */         let res: io::Result<()> = try {
/* FP:linker.rs-1130 */             let mut f = File::create_buffered(&path)?;
/* FP:linker.rs-1131 */ 
/* FP:linker.rs-1132 */             // Start off with the standard module name header and then go
/* FP:linker.rs-1133 */             // straight to exports.
/* FP:linker.rs-1134 */             writeln!(f, "LIBRARY")?;
/* FP:linker.rs-1135 */             writeln!(f, "EXPORTS")?;
/* FP:linker.rs-1136 */             for (symbol, kind) in symbols {
/* FP:linker.rs-1137 */                 let kind_marker = if *kind == SymbolExportKind::Data { " DATA" } else { "" };
/* FP:linker.rs-1138 */                 debug!("  _{symbol}");
/* FP:linker.rs-1139 */                 writeln!(f, "  {symbol}{kind_marker}")?;
/* FP:linker.rs-1140 */             }
/* FP:linker.rs-1141 */         };
/* FP:linker.rs-1142 */         if let Err(error) = res {
/* FP:linker.rs-1143 */             self.sess.dcx().emit_fatal(errors::LibDefWriteFailure { error });
/* FP:linker.rs-1144 */         }
/* FP:linker.rs-1145 */         let mut arg = OsString::from("/DEF:");
/* FP:linker.rs-1146 */         arg.push(path);
/* FP:linker.rs-1147 */         self.link_arg(&arg);
/* FP:linker.rs-1148 */     }
/* FP:linker.rs-1149 */ 
/* FP:linker.rs-1150 */     fn subsystem(&mut self, subsystem: &str) {
/* FP:linker.rs-1151 */         // Note that previous passes of the compiler validated this subsystem,
/* FP:linker.rs-1152 */         // so we just blindly pass it to the linker.
/* FP:linker.rs-1153 */         self.link_arg(&format!("/SUBSYSTEM:{subsystem}"));
/* FP:linker.rs-1154 */ 
/* FP:linker.rs-1155 */         // Windows has two subsystems we're interested in right now, the console
/* FP:linker.rs-1156 */         // and windows subsystems. These both implicitly have different entry
/* FP:linker.rs-1157 */         // points (starting symbols). The console entry point starts with
/* FP:linker.rs-1158 */         // `mainCRTStartup` and the windows entry point starts with
/* FP:linker.rs-1159 */         // `WinMainCRTStartup`. These entry points, defined in system libraries,
/* FP:linker.rs-1160 */         // will then later probe for either `main` or `WinMain`, respectively to
/* FP:linker.rs-1161 */         // start the application.
/* FP:linker.rs-1162 */         //
/* FP:linker.rs-1163 */         // In Rust we just always generate a `main` function so we want control
/* FP:linker.rs-1164 */         // to always start there, so we force the entry point on the windows
/* FP:linker.rs-1165 */         // subsystem to be `mainCRTStartup` to get everything booted up
/* FP:linker.rs-1166 */         // correctly.
/* FP:linker.rs-1167 */         //
/* FP:linker.rs-1168 */         // For more information see RFC #1665
/* FP:linker.rs-1169 */         if subsystem == "windows" {
/* FP:linker.rs-1170 */             self.link_arg("/ENTRY:mainCRTStartup");
/* FP:linker.rs-1171 */         }
/* FP:linker.rs-1172 */     }
/* FP:linker.rs-1173 */ 
/* FP:linker.rs-1174 */     fn linker_plugin_lto(&mut self) {
/* FP:linker.rs-1175 */         // Do nothing
/* FP:linker.rs-1176 */     }
/* FP:linker.rs-1177 */ 
/* FP:linker.rs-1178 */     fn add_no_exec(&mut self) {
/* FP:linker.rs-1179 */         self.link_arg("/NXCOMPAT");
/* FP:linker.rs-1180 */     }
/* FP:linker.rs-1181 */ }
/* FP:linker.rs-1182 */ 
/* FP:linker.rs-1183 */ struct EmLinker<'a> {
/* FP:linker.rs-1184 */     cmd: Command,
/* FP:linker.rs-1185 */     sess: &'a Session,
/* FP:linker.rs-1186 */ }
/* FP:linker.rs-1187 */ 
/* FP:linker.rs-1188 */ impl<'a> Linker for EmLinker<'a> {
/* FP:linker.rs-1189 */     fn cmd(&mut self) -> &mut Command {
/* FP:linker.rs-1190 */         &mut self.cmd
/* FP:linker.rs-1191 */     }
/* FP:linker.rs-1192 */ 
/* FP:linker.rs-1193 */     fn is_cc(&self) -> bool {
/* FP:linker.rs-1194 */         true
/* FP:linker.rs-1195 */     }
/* FP:linker.rs-1196 */ 
/* FP:linker.rs-1197 */     fn set_output_kind(
/* FP:linker.rs-1198 */         &mut self,
/* FP:linker.rs-1199 */         _output_kind: LinkOutputKind,
/* FP:linker.rs-1200 */         _crate_type: CrateType,
/* FP:linker.rs-1201 */         _out_filename: &Path,
/* FP:linker.rs-1202 */     ) {
/* FP:linker.rs-1203 */     }
/* FP:linker.rs-1204 */ 
/* FP:linker.rs-1205 */     fn link_dylib_by_name(&mut self, name: &str, _verbatim: bool, _as_needed: bool) {
/* FP:linker.rs-1206 */         // Emscripten always links statically
/* FP:linker.rs-1207 */         self.link_or_cc_args(&["-l", name]);
/* FP:linker.rs-1208 */     }
/* FP:linker.rs-1209 */ 
/* FP:linker.rs-1210 */     fn link_dylib_by_path(&mut self, path: &Path, _as_needed: bool) {
/* FP:linker.rs-1211 */         self.link_or_cc_arg(path);
/* FP:linker.rs-1212 */     }
/* FP:linker.rs-1213 */ 
/* FP:linker.rs-1214 */     fn link_staticlib_by_name(&mut self, name: &str, _verbatim: bool, _whole_archive: bool) {
/* FP:linker.rs-1215 */         self.link_or_cc_args(&["-l", name]);
/* FP:linker.rs-1216 */     }
/* FP:linker.rs-1217 */ 
/* FP:linker.rs-1218 */     fn link_staticlib_by_path(&mut self, path: &Path, _whole_archive: bool) {
/* FP:linker.rs-1219 */         self.link_or_cc_arg(path);
/* FP:linker.rs-1220 */     }
/* FP:linker.rs-1221 */ 
/* FP:linker.rs-1222 */     fn full_relro(&mut self) {
/* FP:linker.rs-1223 */         // noop
/* FP:linker.rs-1224 */     }
/* FP:linker.rs-1225 */ 
/* FP:linker.rs-1226 */     fn partial_relro(&mut self) {
/* FP:linker.rs-1227 */         // noop
/* FP:linker.rs-1228 */     }
/* FP:linker.rs-1229 */ 
/* FP:linker.rs-1230 */     fn no_relro(&mut self) {
/* FP:linker.rs-1231 */         // noop
/* FP:linker.rs-1232 */     }
/* FP:linker.rs-1233 */ 
/* FP:linker.rs-1234 */     fn gc_sections(&mut self, _keep_metadata: bool) {
/* FP:linker.rs-1235 */         // noop
/* FP:linker.rs-1236 */     }
/* FP:linker.rs-1237 */ 
/* FP:linker.rs-1238 */     fn optimize(&mut self) {
/* FP:linker.rs-1239 */         // Emscripten performs own optimizations
/* FP:linker.rs-1240 */         self.cc_arg(match self.sess.opts.optimize {
/* FP:linker.rs-1241 */             OptLevel::No => "-O0",
/* FP:linker.rs-1242 */             OptLevel::Less => "-O1",
/* FP:linker.rs-1243 */             OptLevel::More => "-O2",
/* FP:linker.rs-1244 */             OptLevel::Aggressive => "-O3",
/* FP:linker.rs-1245 */             OptLevel::Size => "-Os",
/* FP:linker.rs-1246 */             OptLevel::SizeMin => "-Oz",
/* FP:linker.rs-1247 */         });
/* FP:linker.rs-1248 */     }
/* FP:linker.rs-1249 */ 
/* FP:linker.rs-1250 */     fn pgo_gen(&mut self) {
/* FP:linker.rs-1251 */         // noop, but maybe we need something like the gnu linker?
/* FP:linker.rs-1252 */     }
/* FP:linker.rs-1253 */ 
/* FP:linker.rs-1254 */     fn control_flow_guard(&mut self) {}
/* FP:linker.rs-1255 */ 
/* FP:linker.rs-1256 */     fn ehcont_guard(&mut self) {}
/* FP:linker.rs-1257 */ 
/* FP:linker.rs-1258 */     fn debuginfo(&mut self, _strip: Strip, _: &[PathBuf]) {
/* FP:linker.rs-1259 */         // Preserve names or generate source maps depending on debug info
/* FP:linker.rs-1260 */         // For more information see https://emscripten.org/docs/tools_reference/emcc.html#emcc-g
/* FP:linker.rs-1261 */         self.cc_arg(match self.sess.opts.debuginfo {
/* FP:linker.rs-1262 */             DebugInfo::None => "-g0",
/* FP:linker.rs-1263 */             DebugInfo::Limited | DebugInfo::LineTablesOnly | DebugInfo::LineDirectivesOnly => {
/* FP:linker.rs-1264 */                 "--profiling-funcs"
/* FP:linker.rs-1265 */             }
/* FP:linker.rs-1266 */             DebugInfo::Full => "-g",
/* FP:linker.rs-1267 */         });
/* FP:linker.rs-1268 */     }
/* FP:linker.rs-1269 */ 
/* FP:linker.rs-1270 */     fn no_crt_objects(&mut self) {}
/* FP:linker.rs-1271 */ 
/* FP:linker.rs-1272 */     fn no_default_libraries(&mut self) {
/* FP:linker.rs-1273 */         self.cc_arg("-nodefaultlibs");
/* FP:linker.rs-1274 */     }
/* FP:linker.rs-1275 */ 
/* FP:linker.rs-1276 */     fn export_symbols(
/* FP:linker.rs-1277 */         &mut self,
/* FP:linker.rs-1278 */         _tmpdir: &Path,
/* FP:linker.rs-1279 */         _crate_type: CrateType,
/* FP:linker.rs-1280 */         symbols: &[(String, SymbolExportKind)],
/* FP:linker.rs-1281 */     ) {
/* FP:linker.rs-1282 */         debug!("EXPORTED SYMBOLS:");
/* FP:linker.rs-1283 */ 
/* FP:linker.rs-1284 */         self.cc_arg("-s");
/* FP:linker.rs-1285 */ 
/* FP:linker.rs-1286 */         let mut arg = OsString::from("EXPORTED_FUNCTIONS=");
/* FP:linker.rs-1287 */         let encoded = serde_json::to_string(
/* FP:linker.rs-1288 */             &symbols.iter().map(|(sym, _)| "_".to_owned() + sym).collect::<Vec<_>>(),
/* FP:linker.rs-1289 */         )
/* FP:linker.rs-1290 */         .unwrap();
/* FP:linker.rs-1291 */         debug!("{encoded}");
/* FP:linker.rs-1292 */ 
/* FP:linker.rs-1293 */         arg.push(encoded);
/* FP:linker.rs-1294 */ 
/* FP:linker.rs-1295 */         self.cc_arg(arg);
/* FP:linker.rs-1296 */     }
/* FP:linker.rs-1297 */ 
/* FP:linker.rs-1298 */     fn subsystem(&mut self, _subsystem: &str) {
/* FP:linker.rs-1299 */         // noop
/* FP:linker.rs-1300 */     }
/* FP:linker.rs-1301 */ 
/* FP:linker.rs-1302 */     fn linker_plugin_lto(&mut self) {
/* FP:linker.rs-1303 */         // Do nothing
/* FP:linker.rs-1304 */     }
/* FP:linker.rs-1305 */ }
/* FP:linker.rs-1306 */ 
/* FP:linker.rs-1307 */ struct WasmLd<'a> {
/* FP:linker.rs-1308 */     cmd: Command,
/* FP:linker.rs-1309 */     sess: &'a Session,
/* FP:linker.rs-1310 */ }
/* FP:linker.rs-1311 */ 
/* FP:linker.rs-1312 */ impl<'a> WasmLd<'a> {
/* FP:linker.rs-1313 */     fn new(cmd: Command, sess: &'a Session) -> WasmLd<'a> {
/* FP:linker.rs-1314 */         // If the atomics feature is enabled for wasm then we need a whole bunch
/* FP:linker.rs-1315 */         // of flags:
/* FP:linker.rs-1316 */         //
/* FP:linker.rs-1317 */         // * `--shared-memory` - the link won't even succeed without this, flags
/* FP:linker.rs-1318 */         //   the one linear memory as `shared`
/* FP:linker.rs-1319 */         //
/* FP:linker.rs-1320 */         // * `--max-memory=1G` - when specifying a shared memory this must also
/* FP:linker.rs-1321 */         //   be specified. We conservatively choose 1GB but users should be able
/* FP:linker.rs-1322 */         //   to override this with `-C link-arg`.
/* FP:linker.rs-1323 */         //
/* FP:linker.rs-1324 */         // * `--import-memory` - it doesn't make much sense for memory to be
/* FP:linker.rs-1325 */         //   exported in a threaded module because typically you're
/* FP:linker.rs-1326 */         //   sharing memory and instantiating the module multiple times. As a
/* FP:linker.rs-1327 */         //   result if it were exported then we'd just have no sharing.
/* FP:linker.rs-1328 */         //
/* FP:linker.rs-1329 */         // On wasm32-unknown-unknown, we also export symbols for glue code to use:
/* FP:linker.rs-1330 */         //    * `--export=*tls*` - when `#[thread_local]` symbols are used these
/* FP:linker.rs-1331 */         //      symbols are how the TLS segments are initialized and configured.
/* FP:linker.rs-1332 */         let mut wasm_ld = WasmLd { cmd, sess };
/* FP:linker.rs-1333 */         if sess.target_features.contains(&sym::atomics) {
/* FP:linker.rs-1334 */             wasm_ld.link_args(&["--shared-memory", "--max-memory=1073741824", "--import-memory"]);
/* FP:linker.rs-1335 */             if sess.target.os == "unknown" || sess.target.os == "none" {
/* FP:linker.rs-1336 */                 wasm_ld.link_args(&[
/* FP:linker.rs-1337 */                     "--export=__wasm_init_tls",
/* FP:linker.rs-1338 */                     "--export=__tls_size",
/* FP:linker.rs-1339 */                     "--export=__tls_align",
/* FP:linker.rs-1340 */                     "--export=__tls_base",
/* FP:linker.rs-1341 */                 ]);
/* FP:linker.rs-1342 */             }
/* FP:linker.rs-1343 */         }
/* FP:linker.rs-1344 */         wasm_ld
/* FP:linker.rs-1345 */     }
/* FP:linker.rs-1346 */ }
/* FP:linker.rs-1347 */ 
/* FP:linker.rs-1348 */ impl<'a> Linker for WasmLd<'a> {
/* FP:linker.rs-1349 */     fn cmd(&mut self) -> &mut Command {
/* FP:linker.rs-1350 */         &mut self.cmd
/* FP:linker.rs-1351 */     }
/* FP:linker.rs-1352 */ 
/* FP:linker.rs-1353 */     fn set_output_kind(
/* FP:linker.rs-1354 */         &mut self,
/* FP:linker.rs-1355 */         output_kind: LinkOutputKind,
/* FP:linker.rs-1356 */         _crate_type: CrateType,
/* FP:linker.rs-1357 */         _out_filename: &Path,
/* FP:linker.rs-1358 */     ) {
/* FP:linker.rs-1359 */         match output_kind {
/* FP:linker.rs-1360 */             LinkOutputKind::DynamicNoPicExe
/* FP:linker.rs-1361 */             | LinkOutputKind::DynamicPicExe
/* FP:linker.rs-1362 */             | LinkOutputKind::StaticNoPicExe
/* FP:linker.rs-1363 */             | LinkOutputKind::StaticPicExe => {}
/* FP:linker.rs-1364 */             LinkOutputKind::DynamicDylib | LinkOutputKind::StaticDylib => {
/* FP:linker.rs-1365 */                 self.link_arg("--no-entry");
/* FP:linker.rs-1366 */             }
/* FP:linker.rs-1367 */             LinkOutputKind::WasiReactorExe => {
/* FP:linker.rs-1368 */                 self.link_args(&["--entry", "_initialize"]);
/* FP:linker.rs-1369 */             }
/* FP:linker.rs-1370 */         }
/* FP:linker.rs-1371 */     }
/* FP:linker.rs-1372 */ 
/* FP:linker.rs-1373 */     fn link_dylib_by_name(&mut self, name: &str, _verbatim: bool, _as_needed: bool) {
/* FP:linker.rs-1374 */         self.link_or_cc_args(&["-l", name]);
/* FP:linker.rs-1375 */     }
/* FP:linker.rs-1376 */ 
/* FP:linker.rs-1377 */     fn link_dylib_by_path(&mut self, path: &Path, _as_needed: bool) {
/* FP:linker.rs-1378 */         self.link_or_cc_arg(path);
/* FP:linker.rs-1379 */     }
/* FP:linker.rs-1380 */ 
/* FP:linker.rs-1381 */     fn link_staticlib_by_name(&mut self, name: &str, _verbatim: bool, whole_archive: bool) {
/* FP:linker.rs-1382 */         if !whole_archive {
/* FP:linker.rs-1383 */             self.link_or_cc_args(&["-l", name]);
/* FP:linker.rs-1384 */         } else {
/* FP:linker.rs-1385 */             self.link_arg("--whole-archive")
/* FP:linker.rs-1386 */                 .link_or_cc_args(&["-l", name])
/* FP:linker.rs-1387 */                 .link_arg("--no-whole-archive");
/* FP:linker.rs-1388 */         }
/* FP:linker.rs-1389 */     }
/* FP:linker.rs-1390 */ 
/* FP:linker.rs-1391 */     fn link_staticlib_by_path(&mut self, path: &Path, whole_archive: bool) {
/* FP:linker.rs-1392 */         if !whole_archive {
/* FP:linker.rs-1393 */             self.link_or_cc_arg(path);
/* FP:linker.rs-1394 */         } else {
/* FP:linker.rs-1395 */             self.link_arg("--whole-archive").link_or_cc_arg(path).link_arg("--no-whole-archive");
/* FP:linker.rs-1396 */         }
/* FP:linker.rs-1397 */     }
/* FP:linker.rs-1398 */ 
/* FP:linker.rs-1399 */     fn full_relro(&mut self) {}
/* FP:linker.rs-1400 */ 
/* FP:linker.rs-1401 */     fn partial_relro(&mut self) {}
/* FP:linker.rs-1402 */ 
/* FP:linker.rs-1403 */     fn no_relro(&mut self) {}
/* FP:linker.rs-1404 */ 
/* FP:linker.rs-1405 */     fn gc_sections(&mut self, _keep_metadata: bool) {
/* FP:linker.rs-1406 */         self.link_arg("--gc-sections");
/* FP:linker.rs-1407 */     }
/* FP:linker.rs-1408 */ 
/* FP:linker.rs-1409 */     fn optimize(&mut self) {
/* FP:linker.rs-1410 */         // The -O flag is, as of late 2023, only used for merging of strings and debuginfo, and
/* FP:linker.rs-1411 */         // only differentiates -O0 and -O1. It does not apply to LTO.
/* FP:linker.rs-1412 */         self.link_arg(match self.sess.opts.optimize {
/* FP:linker.rs-1413 */             OptLevel::No => "-O0",
/* FP:linker.rs-1414 */             OptLevel::Less => "-O1",
/* FP:linker.rs-1415 */             OptLevel::More => "-O2",
/* FP:linker.rs-1416 */             OptLevel::Aggressive => "-O3",
/* FP:linker.rs-1417 */             // Currently LLD doesn't support `Os` and `Oz`, so pass through `O2`
/* FP:linker.rs-1418 */             // instead.
/* FP:linker.rs-1419 */             OptLevel::Size => "-O2",
/* FP:linker.rs-1420 */             OptLevel::SizeMin => "-O2",
/* FP:linker.rs-1421 */         });
/* FP:linker.rs-1422 */     }
/* FP:linker.rs-1423 */ 
/* FP:linker.rs-1424 */     fn pgo_gen(&mut self) {}
/* FP:linker.rs-1425 */ 
/* FP:linker.rs-1426 */     fn debuginfo(&mut self, strip: Strip, _: &[PathBuf]) {
/* FP:linker.rs-1427 */         match strip {
/* FP:linker.rs-1428 */             Strip::None => {}
/* FP:linker.rs-1429 */             Strip::Debuginfo => {
/* FP:linker.rs-1430 */                 self.link_arg("--strip-debug");
/* FP:linker.rs-1431 */             }
/* FP:linker.rs-1432 */             Strip::Symbols => {
/* FP:linker.rs-1433 */                 self.link_arg("--strip-all");
/* FP:linker.rs-1434 */             }
/* FP:linker.rs-1435 */         }
/* FP:linker.rs-1436 */     }
/* FP:linker.rs-1437 */ 
/* FP:linker.rs-1438 */     fn control_flow_guard(&mut self) {}
/* FP:linker.rs-1439 */ 
/* FP:linker.rs-1440 */     fn ehcont_guard(&mut self) {}
/* FP:linker.rs-1441 */ 
/* FP:linker.rs-1442 */     fn no_crt_objects(&mut self) {}
/* FP:linker.rs-1443 */ 
/* FP:linker.rs-1444 */     fn no_default_libraries(&mut self) {}
/* FP:linker.rs-1445 */ 
/* FP:linker.rs-1446 */     fn export_symbols(
/* FP:linker.rs-1447 */         &mut self,
/* FP:linker.rs-1448 */         _tmpdir: &Path,
/* FP:linker.rs-1449 */         _crate_type: CrateType,
/* FP:linker.rs-1450 */         symbols: &[(String, SymbolExportKind)],
/* FP:linker.rs-1451 */     ) {
/* FP:linker.rs-1452 */         for (sym, _) in symbols {
/* FP:linker.rs-1453 */             self.link_args(&["--export", sym]);
/* FP:linker.rs-1454 */         }
/* FP:linker.rs-1455 */ 
/* FP:linker.rs-1456 */         // LLD will hide these otherwise-internal symbols since it only exports
/* FP:linker.rs-1457 */         // symbols explicitly passed via the `--export` flags above and hides all
/* FP:linker.rs-1458 */         // others. Various bits and pieces of wasm32-unknown-unknown tooling use
/* FP:linker.rs-1459 */         // this, so be sure these symbols make their way out of the linker as well.
/* FP:linker.rs-1460 */         if self.sess.target.os == "unknown" || self.sess.target.os == "none" {
/* FP:linker.rs-1461 */             self.link_args(&["--export=__heap_base", "--export=__data_end"]);
/* FP:linker.rs-1462 */         }
/* FP:linker.rs-1463 */     }
/* FP:linker.rs-1464 */ 
/* FP:linker.rs-1465 */     fn subsystem(&mut self, _subsystem: &str) {}
/* FP:linker.rs-1466 */ 
/* FP:linker.rs-1467 */     fn linker_plugin_lto(&mut self) {
/* FP:linker.rs-1468 */         match self.sess.opts.cg.linker_plugin_lto {
/* FP:linker.rs-1469 */             LinkerPluginLto::Disabled => {
/* FP:linker.rs-1470 */                 // Nothing to do
/* FP:linker.rs-1471 */             }
/* FP:linker.rs-1472 */             LinkerPluginLto::LinkerPluginAuto => {
/* FP:linker.rs-1473 */                 self.push_linker_plugin_lto_args();
/* FP:linker.rs-1474 */             }
/* FP:linker.rs-1475 */             LinkerPluginLto::LinkerPlugin(_) => {
/* FP:linker.rs-1476 */                 self.push_linker_plugin_lto_args();
/* FP:linker.rs-1477 */             }
/* FP:linker.rs-1478 */         }
/* FP:linker.rs-1479 */     }
/* FP:linker.rs-1480 */ }
/* FP:linker.rs-1481 */ 
/* FP:linker.rs-1482 */ impl<'a> WasmLd<'a> {
/* FP:linker.rs-1483 */     fn push_linker_plugin_lto_args(&mut self) {
/* FP:linker.rs-1484 */         let opt_level = match self.sess.opts.optimize {
/* FP:linker.rs-1485 */             config::OptLevel::No => "O0",
/* FP:linker.rs-1486 */             config::OptLevel::Less => "O1",
/* FP:linker.rs-1487 */             config::OptLevel::More => "O2",
/* FP:linker.rs-1488 */             config::OptLevel::Aggressive => "O3",
/* FP:linker.rs-1489 */             // wasm-ld only handles integer LTO opt levels. Use O2
/* FP:linker.rs-1490 */             config::OptLevel::Size | config::OptLevel::SizeMin => "O2",
/* FP:linker.rs-1491 */         };
/* FP:linker.rs-1492 */         self.link_arg(&format!("--lto-{opt_level}"));
/* FP:linker.rs-1493 */     }
/* FP:linker.rs-1494 */ }
/* FP:linker.rs-1495 */ 
/* FP:linker.rs-1496 */ /// Linker shepherd script for L4Re (Fiasco)
/* FP:linker.rs-1497 */ struct L4Bender<'a> {
/* FP:linker.rs-1498 */     cmd: Command,
/* FP:linker.rs-1499 */     sess: &'a Session,
/* FP:linker.rs-1500 */     hinted_static: bool,
/* FP:linker.rs-1501 */ }
/* FP:linker.rs-1502 */ 
/* FP:linker.rs-1503 */ impl<'a> Linker for L4Bender<'a> {
/* FP:linker.rs-1504 */     fn cmd(&mut self) -> &mut Command {
/* FP:linker.rs-1505 */         &mut self.cmd
/* FP:linker.rs-1506 */     }
/* FP:linker.rs-1507 */ 
/* FP:linker.rs-1508 */     fn set_output_kind(
/* FP:linker.rs-1509 */         &mut self,
/* FP:linker.rs-1510 */         _output_kind: LinkOutputKind,
/* FP:linker.rs-1511 */         _crate_type: CrateType,
/* FP:linker.rs-1512 */         _out_filename: &Path,
/* FP:linker.rs-1513 */     ) {
/* FP:linker.rs-1514 */     }
/* FP:linker.rs-1515 */ 
/* FP:linker.rs-1516 */     fn link_staticlib_by_name(&mut self, name: &str, _verbatim: bool, whole_archive: bool) {
/* FP:linker.rs-1517 */         self.hint_static();
/* FP:linker.rs-1518 */         if !whole_archive {
/* FP:linker.rs-1519 */             self.link_arg(format!("-PC{name}"));
/* FP:linker.rs-1520 */         } else {
/* FP:linker.rs-1521 */             self.link_arg("--whole-archive")
/* FP:linker.rs-1522 */                 .link_or_cc_arg(format!("-l{name}"))
/* FP:linker.rs-1523 */                 .link_arg("--no-whole-archive");
/* FP:linker.rs-1524 */         }
/* FP:linker.rs-1525 */     }
/* FP:linker.rs-1526 */ 
/* FP:linker.rs-1527 */     fn link_staticlib_by_path(&mut self, path: &Path, whole_archive: bool) {
/* FP:linker.rs-1528 */         self.hint_static();
/* FP:linker.rs-1529 */         if !whole_archive {
/* FP:linker.rs-1530 */             self.link_or_cc_arg(path);
/* FP:linker.rs-1531 */         } else {
/* FP:linker.rs-1532 */             self.link_arg("--whole-archive").link_or_cc_arg(path).link_arg("--no-whole-archive");
/* FP:linker.rs-1533 */         }
/* FP:linker.rs-1534 */     }
/* FP:linker.rs-1535 */ 
/* FP:linker.rs-1536 */     fn full_relro(&mut self) {
/* FP:linker.rs-1537 */         self.link_args(&["-z", "relro", "-z", "now"]);
/* FP:linker.rs-1538 */     }
/* FP:linker.rs-1539 */ 
/* FP:linker.rs-1540 */     fn partial_relro(&mut self) {
/* FP:linker.rs-1541 */         self.link_args(&["-z", "relro"]);
/* FP:linker.rs-1542 */     }
/* FP:linker.rs-1543 */ 
/* FP:linker.rs-1544 */     fn no_relro(&mut self) {
/* FP:linker.rs-1545 */         self.link_args(&["-z", "norelro"]);
/* FP:linker.rs-1546 */     }
/* FP:linker.rs-1547 */ 
/* FP:linker.rs-1548 */     fn gc_sections(&mut self, keep_metadata: bool) {
/* FP:linker.rs-1549 */         if !keep_metadata {
/* FP:linker.rs-1550 */             self.link_arg("--gc-sections");
/* FP:linker.rs-1551 */         }
/* FP:linker.rs-1552 */     }
/* FP:linker.rs-1553 */ 
/* FP:linker.rs-1554 */     fn optimize(&mut self) {
/* FP:linker.rs-1555 */         // GNU-style linkers support optimization with -O. GNU ld doesn't
/* FP:linker.rs-1556 */         // need a numeric argument, but other linkers do.
/* FP:linker.rs-1557 */         if self.sess.opts.optimize == config::OptLevel::More
/* FP:linker.rs-1558 */             || self.sess.opts.optimize == config::OptLevel::Aggressive
/* FP:linker.rs-1559 */         {
/* FP:linker.rs-1560 */             self.link_arg("-O1");
/* FP:linker.rs-1561 */         }
/* FP:linker.rs-1562 */     }
/* FP:linker.rs-1563 */ 
/* FP:linker.rs-1564 */     fn pgo_gen(&mut self) {}
/* FP:linker.rs-1565 */ 
/* FP:linker.rs-1566 */     fn debuginfo(&mut self, strip: Strip, _: &[PathBuf]) {
/* FP:linker.rs-1567 */         match strip {
/* FP:linker.rs-1568 */             Strip::None => {}
/* FP:linker.rs-1569 */             Strip::Debuginfo => {
/* FP:linker.rs-1570 */                 self.link_arg("--strip-debug");
/* FP:linker.rs-1571 */             }
/* FP:linker.rs-1572 */             Strip::Symbols => {
/* FP:linker.rs-1573 */                 self.link_arg("--strip-all");
/* FP:linker.rs-1574 */             }
/* FP:linker.rs-1575 */         }
/* FP:linker.rs-1576 */     }
/* FP:linker.rs-1577 */ 
/* FP:linker.rs-1578 */     fn no_default_libraries(&mut self) {
/* FP:linker.rs-1579 */         self.cc_arg("-nostdlib");
/* FP:linker.rs-1580 */     }
/* FP:linker.rs-1581 */ 
/* FP:linker.rs-1582 */     fn export_symbols(&mut self, _: &Path, _: CrateType, _: &[(String, SymbolExportKind)]) {
/* FP:linker.rs-1583 */         // ToDo, not implemented, copy from GCC
/* FP:linker.rs-1584 */         self.sess.dcx().emit_warn(errors::L4BenderExportingSymbolsUnimplemented);
/* FP:linker.rs-1585 */     }
/* FP:linker.rs-1586 */ 
/* FP:linker.rs-1587 */     fn subsystem(&mut self, subsystem: &str) {
/* FP:linker.rs-1588 */         self.link_arg(&format!("--subsystem {subsystem}"));
/* FP:linker.rs-1589 */     }
/* FP:linker.rs-1590 */ 
/* FP:linker.rs-1591 */     fn reset_per_library_state(&mut self) {
/* FP:linker.rs-1592 */         self.hint_static(); // Reset to default before returning the composed command line.
/* FP:linker.rs-1593 */     }
/* FP:linker.rs-1594 */ 
/* FP:linker.rs-1595 */     fn linker_plugin_lto(&mut self) {}
/* FP:linker.rs-1596 */ 
/* FP:linker.rs-1597 */     fn control_flow_guard(&mut self) {}
/* FP:linker.rs-1598 */ 
/* FP:linker.rs-1599 */     fn ehcont_guard(&mut self) {}
/* FP:linker.rs-1600 */ 
/* FP:linker.rs-1601 */     fn no_crt_objects(&mut self) {}
/* FP:linker.rs-1602 */ }
/* FP:linker.rs-1603 */ 
/* FP:linker.rs-1604 */ impl<'a> L4Bender<'a> {
/* FP:linker.rs-1605 */     fn new(cmd: Command, sess: &'a Session) -> L4Bender<'a> {
/* FP:linker.rs-1606 */         L4Bender { cmd, sess, hinted_static: false }
/* FP:linker.rs-1607 */     }
/* FP:linker.rs-1608 */ 
/* FP:linker.rs-1609 */     fn hint_static(&mut self) {
/* FP:linker.rs-1610 */         if !self.hinted_static {
/* FP:linker.rs-1611 */             self.link_or_cc_arg("-static");
/* FP:linker.rs-1612 */             self.hinted_static = true;
/* FP:linker.rs-1613 */         }
/* FP:linker.rs-1614 */     }
/* FP:linker.rs-1615 */ }
/* FP:linker.rs-1616 */ 
/* FP:linker.rs-1617 */ /// Linker for AIX.
/* FP:linker.rs-1618 */ struct AixLinker<'a> {
/* FP:linker.rs-1619 */     cmd: Command,
/* FP:linker.rs-1620 */     sess: &'a Session,
/* FP:linker.rs-1621 */     hinted_static: Option<bool>,
/* FP:linker.rs-1622 */ }
/* FP:linker.rs-1623 */ 
/* FP:linker.rs-1624 */ impl<'a> AixLinker<'a> {
/* FP:linker.rs-1625 */     fn new(cmd: Command, sess: &'a Session) -> AixLinker<'a> {
/* FP:linker.rs-1626 */         AixLinker { cmd, sess, hinted_static: None }
/* FP:linker.rs-1627 */     }
/* FP:linker.rs-1628 */ 
/* FP:linker.rs-1629 */     fn hint_static(&mut self) {
/* FP:linker.rs-1630 */         if self.hinted_static != Some(true) {
/* FP:linker.rs-1631 */             self.link_arg("-bstatic");
/* FP:linker.rs-1632 */             self.hinted_static = Some(true);
/* FP:linker.rs-1633 */         }
/* FP:linker.rs-1634 */     }
/* FP:linker.rs-1635 */ 
/* FP:linker.rs-1636 */     fn hint_dynamic(&mut self) {
/* FP:linker.rs-1637 */         if self.hinted_static != Some(false) {
/* FP:linker.rs-1638 */             self.link_arg("-bdynamic");
/* FP:linker.rs-1639 */             self.hinted_static = Some(false);
/* FP:linker.rs-1640 */         }
/* FP:linker.rs-1641 */     }
/* FP:linker.rs-1642 */ 
/* FP:linker.rs-1643 */     fn build_dylib(&mut self, _out_filename: &Path) {
/* FP:linker.rs-1644 */         self.link_args(&["-bM:SRE", "-bnoentry"]);
/* FP:linker.rs-1645 */         // FIXME: Use CreateExportList utility to create export list
/* FP:linker.rs-1646 */         // and remove -bexpfull.
/* FP:linker.rs-1647 */         self.link_arg("-bexpfull");
/* FP:linker.rs-1648 */     }
/* FP:linker.rs-1649 */ }
/* FP:linker.rs-1650 */ 
/* FP:linker.rs-1651 */ impl<'a> Linker for AixLinker<'a> {
/* FP:linker.rs-1652 */     fn cmd(&mut self) -> &mut Command {
/* FP:linker.rs-1653 */         &mut self.cmd
/* FP:linker.rs-1654 */     }
/* FP:linker.rs-1655 */ 
/* FP:linker.rs-1656 */     fn set_output_kind(
/* FP:linker.rs-1657 */         &mut self,
/* FP:linker.rs-1658 */         output_kind: LinkOutputKind,
/* FP:linker.rs-1659 */         _crate_type: CrateType,
/* FP:linker.rs-1660 */         out_filename: &Path,
/* FP:linker.rs-1661 */     ) {
/* FP:linker.rs-1662 */         match output_kind {
/* FP:linker.rs-1663 */             LinkOutputKind::DynamicDylib => {
/* FP:linker.rs-1664 */                 self.hint_dynamic();
/* FP:linker.rs-1665 */                 self.build_dylib(out_filename);
/* FP:linker.rs-1666 */             }
/* FP:linker.rs-1667 */             LinkOutputKind::StaticDylib => {
/* FP:linker.rs-1668 */                 self.hint_static();
/* FP:linker.rs-1669 */                 self.build_dylib(out_filename);
/* FP:linker.rs-1670 */             }
/* FP:linker.rs-1671 */             _ => {}
/* FP:linker.rs-1672 */         }
/* FP:linker.rs-1673 */     }
/* FP:linker.rs-1674 */ 
/* FP:linker.rs-1675 */     fn link_dylib_by_name(&mut self, name: &str, verbatim: bool, _as_needed: bool) {
/* FP:linker.rs-1676 */         self.hint_dynamic();
/* FP:linker.rs-1677 */         self.link_or_cc_arg(if verbatim { String::from(name) } else { format!("-l{name}") });
/* FP:linker.rs-1678 */     }
/* FP:linker.rs-1679 */ 
/* FP:linker.rs-1680 */     fn link_dylib_by_path(&mut self, path: &Path, _as_needed: bool) {
/* FP:linker.rs-1681 */         self.hint_dynamic();
/* FP:linker.rs-1682 */         self.link_or_cc_arg(path);
/* FP:linker.rs-1683 */     }
/* FP:linker.rs-1684 */ 
/* FP:linker.rs-1685 */     fn link_staticlib_by_name(&mut self, name: &str, verbatim: bool, whole_archive: bool) {
/* FP:linker.rs-1686 */         self.hint_static();
/* FP:linker.rs-1687 */         if !whole_archive {
/* FP:linker.rs-1688 */             self.link_or_cc_arg(if verbatim { String::from(name) } else { format!("-l{name}") });
/* FP:linker.rs-1689 */         } else {
/* FP:linker.rs-1690 */             let mut arg = OsString::from("-bkeepfile:");
/* FP:linker.rs-1691 */             arg.push(find_native_static_library(name, verbatim, self.sess));
/* FP:linker.rs-1692 */             self.link_or_cc_arg(arg);
/* FP:linker.rs-1693 */         }
/* FP:linker.rs-1694 */     }
/* FP:linker.rs-1695 */ 
/* FP:linker.rs-1696 */     fn link_staticlib_by_path(&mut self, path: &Path, whole_archive: bool) {
/* FP:linker.rs-1697 */         self.hint_static();
/* FP:linker.rs-1698 */         if !whole_archive {
/* FP:linker.rs-1699 */             self.link_or_cc_arg(path);
/* FP:linker.rs-1700 */         } else {
/* FP:linker.rs-1701 */             let mut arg = OsString::from("-bkeepfile:");
/* FP:linker.rs-1702 */             arg.push(path);
/* FP:linker.rs-1703 */             self.link_arg(arg);
/* FP:linker.rs-1704 */         }
/* FP:linker.rs-1705 */     }
/* FP:linker.rs-1706 */ 
/* FP:linker.rs-1707 */     fn full_relro(&mut self) {}
/* FP:linker.rs-1708 */ 
/* FP:linker.rs-1709 */     fn partial_relro(&mut self) {}
/* FP:linker.rs-1710 */ 
/* FP:linker.rs-1711 */     fn no_relro(&mut self) {}
/* FP:linker.rs-1712 */ 
/* FP:linker.rs-1713 */     fn gc_sections(&mut self, _keep_metadata: bool) {
/* FP:linker.rs-1714 */         self.link_arg("-bgc");
/* FP:linker.rs-1715 */     }
/* FP:linker.rs-1716 */ 
/* FP:linker.rs-1717 */     fn optimize(&mut self) {}
/* FP:linker.rs-1718 */ 
/* FP:linker.rs-1719 */     fn pgo_gen(&mut self) {
/* FP:linker.rs-1720 */         self.link_arg("-bdbg:namedsects:ss");
/* FP:linker.rs-1721 */         self.link_arg("-u");
/* FP:linker.rs-1722 */         self.link_arg("__llvm_profile_runtime");
/* FP:linker.rs-1723 */     }
/* FP:linker.rs-1724 */ 
/* FP:linker.rs-1725 */     fn control_flow_guard(&mut self) {}
/* FP:linker.rs-1726 */ 
/* FP:linker.rs-1727 */     fn ehcont_guard(&mut self) {}
/* FP:linker.rs-1728 */ 
/* FP:linker.rs-1729 */     fn debuginfo(&mut self, _: Strip, _: &[PathBuf]) {}
/* FP:linker.rs-1730 */ 
/* FP:linker.rs-1731 */     fn no_crt_objects(&mut self) {}
/* FP:linker.rs-1732 */ 
/* FP:linker.rs-1733 */     fn no_default_libraries(&mut self) {}
/* FP:linker.rs-1734 */ 
/* FP:linker.rs-1735 */     fn export_symbols(
/* FP:linker.rs-1736 */         &mut self,
/* FP:linker.rs-1737 */         tmpdir: &Path,
/* FP:linker.rs-1738 */         _crate_type: CrateType,
/* FP:linker.rs-1739 */         symbols: &[(String, SymbolExportKind)],
/* FP:linker.rs-1740 */     ) {
/* FP:linker.rs-1741 */         let path = tmpdir.join("list.exp");
/* FP:linker.rs-1742 */         let res: io::Result<()> = try {
/* FP:linker.rs-1743 */             let mut f = File::create_buffered(&path)?;
/* FP:linker.rs-1744 */             // FIXME: use llvm-nm to generate export list.
/* FP:linker.rs-1745 */             for (symbol, _) in symbols {
/* FP:linker.rs-1746 */                 debug!("  _{symbol}");
/* FP:linker.rs-1747 */                 writeln!(f, "  {symbol}")?;
/* FP:linker.rs-1748 */             }
/* FP:linker.rs-1749 */         };
/* FP:linker.rs-1750 */         if let Err(e) = res {
/* FP:linker.rs-1751 */             self.sess.dcx().fatal(format!("failed to write export file: {e}"));
/* FP:linker.rs-1752 */         }
/* FP:linker.rs-1753 */         self.link_arg(format!("-bE:{}", path.to_str().unwrap()));
/* FP:linker.rs-1754 */     }
/* FP:linker.rs-1755 */ 
/* FP:linker.rs-1756 */     fn subsystem(&mut self, _subsystem: &str) {}
/* FP:linker.rs-1757 */ 
/* FP:linker.rs-1758 */     fn reset_per_library_state(&mut self) {
/* FP:linker.rs-1759 */         self.hint_dynamic();
/* FP:linker.rs-1760 */     }
/* FP:linker.rs-1761 */ 
/* FP:linker.rs-1762 */     fn linker_plugin_lto(&mut self) {}
/* FP:linker.rs-1763 */ 
/* FP:linker.rs-1764 */     fn add_eh_frame_header(&mut self) {}
/* FP:linker.rs-1765 */ 
/* FP:linker.rs-1766 */     fn add_no_exec(&mut self) {}
/* FP:linker.rs-1767 */ 
/* FP:linker.rs-1768 */     fn add_as_needed(&mut self) {}
/* FP:linker.rs-1769 */ }
/* FP:linker.rs-1770 */ 
/* FP:linker.rs-1771 */ fn for_each_exported_symbols_include_dep<'tcx>(
/* FP:linker.rs-1772 */     tcx: TyCtxt<'tcx>,
/* FP:linker.rs-1773 */     crate_type: CrateType,
/* FP:linker.rs-1774 */     mut callback: impl FnMut(ExportedSymbol<'tcx>, SymbolExportInfo, CrateNum),
/* FP:linker.rs-1775 */ ) {
/* FP:linker.rs-1776 */     let formats = tcx.dependency_formats(());
/* FP:linker.rs-1777 */     let deps = &formats[&crate_type];
/* FP:linker.rs-1778 */ 
/* FP:linker.rs-1779 */     for (cnum, dep_format) in deps.iter_enumerated() {
/* FP:linker.rs-1780 */         // For each dependency that we are linking to statically ...
/* FP:linker.rs-1781 */         if *dep_format == Linkage::Static {
/* FP:linker.rs-1782 */             for &(symbol, info) in tcx.exported_non_generic_symbols(cnum).iter() {
/* FP:linker.rs-1783 */                 callback(symbol, info, cnum);
/* FP:linker.rs-1784 */             }
/* FP:linker.rs-1785 */             for &(symbol, info) in tcx.exported_generic_symbols(cnum).iter() {
/* FP:linker.rs-1786 */                 callback(symbol, info, cnum);
/* FP:linker.rs-1787 */             }
/* FP:linker.rs-1788 */         }
/* FP:linker.rs-1789 */     }
/* FP:linker.rs-1790 */ }
/* FP:linker.rs-1791 */ 
/* FP:linker.rs-1792 */ pub(crate) fn exported_symbols(
/* FP:linker.rs-1793 */     tcx: TyCtxt<'_>,
/* FP:linker.rs-1794 */     crate_type: CrateType,
/* FP:linker.rs-1795 */ ) -> Vec<(String, SymbolExportKind)> {
/* FP:linker.rs-1796 */     if let Some(ref exports) = tcx.sess.target.override_export_symbols {
/* FP:linker.rs-1797 */         return exports
/* FP:linker.rs-1798 */             .iter()
/* FP:linker.rs-1799 */             .map(|name| {
/* FP:linker.rs-1800 */                 (
/* FP:linker.rs-1801 */                     name.to_string(),
/* FP:linker.rs-1802 */                     // FIXME use the correct export kind for this symbol. override_export_symbols
/* FP:linker.rs-1803 */                     // can't directly specify the SymbolExportKind as it is defined in rustc_middle
/* FP:linker.rs-1804 */                     // which rustc_target can't depend on.
/* FP:linker.rs-1805 */                     SymbolExportKind::Text,
/* FP:linker.rs-1806 */                 )
/* FP:linker.rs-1807 */             })
/* FP:linker.rs-1808 */             .collect();
/* FP:linker.rs-1809 */     }
/* FP:linker.rs-1810 */ 
/* FP:linker.rs-1811 */     let mut symbols = if let CrateType::ProcMacro = crate_type {
/* FP:linker.rs-1812 */         exported_symbols_for_proc_macro_crate(tcx)
/* FP:linker.rs-1813 */     } else {
/* FP:linker.rs-1814 */         exported_symbols_for_non_proc_macro(tcx, crate_type)
/* FP:linker.rs-1815 */     };
/* FP:linker.rs-1816 */ 
/* FP:linker.rs-1817 */     if crate_type == CrateType::Dylib || crate_type == CrateType::ProcMacro {
/* FP:linker.rs-1818 */         let metadata_symbol_name = exported_symbols::metadata_symbol_name(tcx);
/* FP:linker.rs-1819 */         symbols.push((metadata_symbol_name, SymbolExportKind::Data));
/* FP:linker.rs-1820 */     }
/* FP:linker.rs-1821 */ 
/* FP:linker.rs-1822 */     symbols
/* FP:linker.rs-1823 */ }
/* FP:linker.rs-1824 */ 
/* FP:linker.rs-1825 */ fn exported_symbols_for_non_proc_macro(
/* FP:linker.rs-1826 */     tcx: TyCtxt<'_>,
/* FP:linker.rs-1827 */     crate_type: CrateType,
/* FP:linker.rs-1828 */ ) -> Vec<(String, SymbolExportKind)> {
/* FP:linker.rs-1829 */     let mut symbols = Vec::new();
/* FP:linker.rs-1830 */     let export_threshold = symbol_export::crates_export_threshold(&[crate_type]);
/* FP:linker.rs-1831 */     for_each_exported_symbols_include_dep(tcx, crate_type, |symbol, info, cnum| {
/* FP:linker.rs-1832 */         // Do not export mangled symbols from cdylibs and don't attempt to export compiler-builtins
/* FP:linker.rs-1833 */         // from any dylib. The latter doesn't work anyway as we use hidden visibility for
/* FP:linker.rs-1834 */         // compiler-builtins. Most linkers silently ignore it, but ld64 gives a warning.
/* FP:linker.rs-1835 */         if info.level.is_below_threshold(export_threshold) && !tcx.is_compiler_builtins(cnum) {
/* FP:linker.rs-1836 */             symbols.push((
/* FP:linker.rs-1837 */                 symbol_export::exporting_symbol_name_for_instance_in_crate(tcx, symbol, cnum),
/* FP:linker.rs-1838 */                 info.kind,
/* FP:linker.rs-1839 */             ));
/* FP:linker.rs-1840 */             symbol_export::extend_exported_symbols(&mut symbols, tcx, symbol, cnum);
/* FP:linker.rs-1841 */         }
/* FP:linker.rs-1842 */     });
/* FP:linker.rs-1843 */ 
/* FP:linker.rs-1844 */     // Mark allocator shim symbols as exported only if they were generated.
/* FP:linker.rs-1845 */     if export_threshold == SymbolExportLevel::Rust
/* FP:linker.rs-1846 */         && needs_allocator_shim_for_linking(tcx.dependency_formats(()), crate_type)
/* FP:linker.rs-1847 */         && tcx.allocator_kind(()).is_some()
/* FP:linker.rs-1848 */     {
/* FP:linker.rs-1849 */         symbols.extend(allocator_shim_symbols(tcx));
/* FP:linker.rs-1850 */     }
/* FP:linker.rs-1851 */ 
/* FP:linker.rs-1852 */     symbols
/* FP:linker.rs-1853 */ }
/* FP:linker.rs-1854 */ 
/* FP:linker.rs-1855 */ fn exported_symbols_for_proc_macro_crate(tcx: TyCtxt<'_>) -> Vec<(String, SymbolExportKind)> {
/* FP:linker.rs-1856 */     // `exported_symbols` will be empty when !should_codegen.
/* FP:linker.rs-1857 */     if !tcx.sess.opts.output_types.should_codegen() {
/* FP:linker.rs-1858 */         return Vec::new();
/* FP:linker.rs-1859 */     }
/* FP:linker.rs-1860 */ 
/* FP:linker.rs-1861 */     let stable_crate_id = tcx.stable_crate_id(LOCAL_CRATE);
/* FP:linker.rs-1862 */     let proc_macro_decls_name = tcx.sess.generate_proc_macro_decls_symbol(stable_crate_id);
/* FP:linker.rs-1863 */ 
/* FP:linker.rs-1864 */     vec![(proc_macro_decls_name, SymbolExportKind::Data)]
/* FP:linker.rs-1865 */ }
/* FP:linker.rs-1866 */ 
/* FP:linker.rs-1867 */ pub(crate) fn linked_symbols(
/* FP:linker.rs-1868 */     tcx: TyCtxt<'_>,
/* FP:linker.rs-1869 */     crate_type: CrateType,
/* FP:linker.rs-1870 */ ) -> Vec<(String, SymbolExportKind)> {
/* FP:linker.rs-1871 */     match crate_type {
/* FP:linker.rs-1872 */         CrateType::Executable
/* FP:linker.rs-1873 */         | CrateType::ProcMacro
/* FP:linker.rs-1874 */         | CrateType::Cdylib
/* FP:linker.rs-1875 */         | CrateType::Dylib
/* FP:linker.rs-1876 */         | CrateType::Sdylib => (),
/* FP:linker.rs-1877 */         CrateType::Staticlib | CrateType::Rlib => {
/* FP:linker.rs-1878 */             // These are not linked, so no need to generate symbols.o for them.
/* FP:linker.rs-1879 */             return Vec::new();
/* FP:linker.rs-1880 */         }
/* FP:linker.rs-1881 */     }
/* FP:linker.rs-1882 */ 
/* FP:linker.rs-1883 */     match tcx.sess.lto() {
/* FP:linker.rs-1884 */         Lto::No | Lto::ThinLocal => {}
/* FP:linker.rs-1885 */         Lto::Thin | Lto::Fat => {
/* FP:linker.rs-1886 */             // We really only need symbols from upstream rlibs to end up in the linked symbols list.
/* FP:linker.rs-1887 */             // The rest are in separate object files which the linker will always link in and
/* FP:linker.rs-1888 */             // doesn't have rules around the order in which they need to appear.
/* FP:linker.rs-1889 */             // When doing LTO, some of the symbols in the linked symbols list happen to be
/* FP:linker.rs-1890 */             // internalized by LTO, which then prevents referencing them from symbols.o. When doing
/* FP:linker.rs-1891 */             // LTO, all object files that get linked in will be local object files rather than
/* FP:linker.rs-1892 */             // pulled in from rlibs, so an empty linked symbols list works fine to avoid referencing
/* FP:linker.rs-1893 */             // all those internalized symbols from symbols.o.
/* FP:linker.rs-1894 */             return Vec::new();
/* FP:linker.rs-1895 */         }
/* FP:linker.rs-1896 */     }
/* FP:linker.rs-1897 */ 
/* FP:linker.rs-1898 */     let mut symbols = Vec::new();
/* FP:linker.rs-1899 */ 
/* FP:linker.rs-1900 */     let export_threshold = symbol_export::crates_export_threshold(&[crate_type]);
/* FP:linker.rs-1901 */     for_each_exported_symbols_include_dep(tcx, crate_type, |symbol, info, cnum| {
/* FP:linker.rs-1902 */         if info.level.is_below_threshold(export_threshold) && !tcx.is_compiler_builtins(cnum)
/* FP:linker.rs-1903 */             || info.used
/* FP:linker.rs-1904 */             || info.rustc_std_internal_symbol
/* FP:linker.rs-1905 */         {
/* FP:linker.rs-1906 */             symbols.push((
/* FP:linker.rs-1907 */                 symbol_export::linking_symbol_name_for_instance_in_crate(
/* FP:linker.rs-1908 */                     tcx, symbol, info.kind, cnum,
/* FP:linker.rs-1909 */                 ),
/* FP:linker.rs-1910 */                 info.kind,
/* FP:linker.rs-1911 */             ));
/* FP:linker.rs-1912 */         }
/* FP:linker.rs-1913 */     });
/* FP:linker.rs-1914 */ 
/* FP:linker.rs-1915 */     symbols
/* FP:linker.rs-1916 */ }
/* FP:linker.rs-1917 */ 
/* FP:linker.rs-1918 */ /// Much simplified and explicit CLI for the NVPTX linker. The linker operates
/* FP:linker.rs-1919 */ /// with bitcode and uses LLVM backend to generate a PTX assembly.
/* FP:linker.rs-1920 */ struct PtxLinker<'a> {
/* FP:linker.rs-1921 */     cmd: Command,
/* FP:linker.rs-1922 */     sess: &'a Session,
/* FP:linker.rs-1923 */ }
/* FP:linker.rs-1924 */ 
/* FP:linker.rs-1925 */ impl<'a> Linker for PtxLinker<'a> {
/* FP:linker.rs-1926 */     fn cmd(&mut self) -> &mut Command {
/* FP:linker.rs-1927 */         &mut self.cmd
/* FP:linker.rs-1928 */     }
/* FP:linker.rs-1929 */ 
/* FP:linker.rs-1930 */     fn set_output_kind(
/* FP:linker.rs-1931 */         &mut self,
/* FP:linker.rs-1932 */         _output_kind: LinkOutputKind,
/* FP:linker.rs-1933 */         _crate_type: CrateType,
/* FP:linker.rs-1934 */         _out_filename: &Path,
/* FP:linker.rs-1935 */     ) {
/* FP:linker.rs-1936 */     }
/* FP:linker.rs-1937 */ 
/* FP:linker.rs-1938 */     fn link_staticlib_by_name(&mut self, _name: &str, _verbatim: bool, _whole_archive: bool) {
/* FP:linker.rs-1939 */         panic!("staticlibs not supported")
/* FP:linker.rs-1940 */     }
/* FP:linker.rs-1941 */ 
/* FP:linker.rs-1942 */     fn link_staticlib_by_path(&mut self, path: &Path, _whole_archive: bool) {
/* FP:linker.rs-1943 */         self.link_arg("--rlib").link_arg(path);
/* FP:linker.rs-1944 */     }
/* FP:linker.rs-1945 */ 
/* FP:linker.rs-1946 */     fn debuginfo(&mut self, _strip: Strip, _: &[PathBuf]) {
/* FP:linker.rs-1947 */         self.link_arg("--debug");
/* FP:linker.rs-1948 */     }
/* FP:linker.rs-1949 */ 
/* FP:linker.rs-1950 */     fn add_object(&mut self, path: &Path) {
/* FP:linker.rs-1951 */         self.link_arg("--bitcode").link_arg(path);
/* FP:linker.rs-1952 */     }
/* FP:linker.rs-1953 */ 
/* FP:linker.rs-1954 */     fn optimize(&mut self) {
/* FP:linker.rs-1955 */         match self.sess.lto() {
/* FP:linker.rs-1956 */             Lto::Thin | Lto::Fat | Lto::ThinLocal => {
/* FP:linker.rs-1957 */                 self.link_arg("-Olto");
/* FP:linker.rs-1958 */             }
/* FP:linker.rs-1959 */ 
/* FP:linker.rs-1960 */             Lto::No => {}
/* FP:linker.rs-1961 */         }
/* FP:linker.rs-1962 */     }
/* FP:linker.rs-1963 */ 
/* FP:linker.rs-1964 */     fn full_relro(&mut self) {}
/* FP:linker.rs-1965 */ 
/* FP:linker.rs-1966 */     fn partial_relro(&mut self) {}
/* FP:linker.rs-1967 */ 
/* FP:linker.rs-1968 */     fn no_relro(&mut self) {}
/* FP:linker.rs-1969 */ 
/* FP:linker.rs-1970 */     fn gc_sections(&mut self, _keep_metadata: bool) {}
/* FP:linker.rs-1971 */ 
/* FP:linker.rs-1972 */     fn pgo_gen(&mut self) {}
/* FP:linker.rs-1973 */ 
/* FP:linker.rs-1974 */     fn no_crt_objects(&mut self) {}
/* FP:linker.rs-1975 */ 
/* FP:linker.rs-1976 */     fn no_default_libraries(&mut self) {}
/* FP:linker.rs-1977 */ 
/* FP:linker.rs-1978 */     fn control_flow_guard(&mut self) {}
/* FP:linker.rs-1979 */ 
/* FP:linker.rs-1980 */     fn ehcont_guard(&mut self) {}
/* FP:linker.rs-1981 */ 
/* FP:linker.rs-1982 */     fn export_symbols(
/* FP:linker.rs-1983 */         &mut self,
/* FP:linker.rs-1984 */         _tmpdir: &Path,
/* FP:linker.rs-1985 */         _crate_type: CrateType,
/* FP:linker.rs-1986 */         _symbols: &[(String, SymbolExportKind)],
/* FP:linker.rs-1987 */     ) {
/* FP:linker.rs-1988 */     }
/* FP:linker.rs-1989 */ 
/* FP:linker.rs-1990 */     fn subsystem(&mut self, _subsystem: &str) {}
/* FP:linker.rs-1991 */ 
/* FP:linker.rs-1992 */     fn linker_plugin_lto(&mut self) {}
/* FP:linker.rs-1993 */ }
/* FP:linker.rs-1994 */ 
/* FP:linker.rs-1995 */ /// The `self-contained` LLVM bitcode linker
/* FP:linker.rs-1996 */ struct LlbcLinker<'a> {
/* FP:linker.rs-1997 */     cmd: Command,
/* FP:linker.rs-1998 */     sess: &'a Session,
/* FP:linker.rs-1999 */ }
/* FP:linker.rs-2000 */ 
/* FP:linker.rs-2001 */ impl<'a> Linker for LlbcLinker<'a> {
/* FP:linker.rs-2002 */     fn cmd(&mut self) -> &mut Command {
/* FP:linker.rs-2003 */         &mut self.cmd
/* FP:linker.rs-2004 */     }
/* FP:linker.rs-2005 */ 
/* FP:linker.rs-2006 */     fn set_output_kind(
/* FP:linker.rs-2007 */         &mut self,
/* FP:linker.rs-2008 */         _output_kind: LinkOutputKind,
/* FP:linker.rs-2009 */         _crate_type: CrateType,
/* FP:linker.rs-2010 */         _out_filename: &Path,
/* FP:linker.rs-2011 */     ) {
/* FP:linker.rs-2012 */     }
/* FP:linker.rs-2013 */ 
/* FP:linker.rs-2014 */     fn link_staticlib_by_name(&mut self, _name: &str, _verbatim: bool, _whole_archive: bool) {
/* FP:linker.rs-2015 */         panic!("staticlibs not supported")
/* FP:linker.rs-2016 */     }
/* FP:linker.rs-2017 */ 
/* FP:linker.rs-2018 */     fn link_staticlib_by_path(&mut self, path: &Path, _whole_archive: bool) {
/* FP:linker.rs-2019 */         self.link_or_cc_arg(path);
/* FP:linker.rs-2020 */     }
/* FP:linker.rs-2021 */ 
/* FP:linker.rs-2022 */     fn debuginfo(&mut self, _strip: Strip, _: &[PathBuf]) {
/* FP:linker.rs-2023 */         self.link_arg("--debug");
/* FP:linker.rs-2024 */     }
/* FP:linker.rs-2025 */ 
/* FP:linker.rs-2026 */     fn optimize(&mut self) {
/* FP:linker.rs-2027 */         self.link_arg(match self.sess.opts.optimize {
/* FP:linker.rs-2028 */             OptLevel::No => "-O0",
/* FP:linker.rs-2029 */             OptLevel::Less => "-O1",
/* FP:linker.rs-2030 */             OptLevel::More => "-O2",
/* FP:linker.rs-2031 */             OptLevel::Aggressive => "-O3",
/* FP:linker.rs-2032 */             OptLevel::Size => "-Os",
/* FP:linker.rs-2033 */             OptLevel::SizeMin => "-Oz",
/* FP:linker.rs-2034 */         });
/* FP:linker.rs-2035 */     }
/* FP:linker.rs-2036 */ 
/* FP:linker.rs-2037 */     fn full_relro(&mut self) {}
/* FP:linker.rs-2038 */ 
/* FP:linker.rs-2039 */     fn partial_relro(&mut self) {}
/* FP:linker.rs-2040 */ 
/* FP:linker.rs-2041 */     fn no_relro(&mut self) {}
/* FP:linker.rs-2042 */ 
/* FP:linker.rs-2043 */     fn gc_sections(&mut self, _keep_metadata: bool) {}
/* FP:linker.rs-2044 */ 
/* FP:linker.rs-2045 */     fn pgo_gen(&mut self) {}
/* FP:linker.rs-2046 */ 
/* FP:linker.rs-2047 */     fn no_crt_objects(&mut self) {}
/* FP:linker.rs-2048 */ 
/* FP:linker.rs-2049 */     fn no_default_libraries(&mut self) {}
/* FP:linker.rs-2050 */ 
/* FP:linker.rs-2051 */     fn control_flow_guard(&mut self) {}
/* FP:linker.rs-2052 */ 
/* FP:linker.rs-2053 */     fn ehcont_guard(&mut self) {}
/* FP:linker.rs-2054 */ 
/* FP:linker.rs-2055 */     fn export_symbols(
/* FP:linker.rs-2056 */         &mut self,
/* FP:linker.rs-2057 */         _tmpdir: &Path,
/* FP:linker.rs-2058 */         _crate_type: CrateType,
/* FP:linker.rs-2059 */         symbols: &[(String, SymbolExportKind)],
/* FP:linker.rs-2060 */     ) {
/* FP:linker.rs-2061 */         match _crate_type {
/* FP:linker.rs-2062 */             CrateType::Cdylib => {
/* FP:linker.rs-2063 */                 for (sym, _) in symbols {
/* FP:linker.rs-2064 */                     self.link_args(&["--export-symbol", sym]);
/* FP:linker.rs-2065 */                 }
/* FP:linker.rs-2066 */             }
/* FP:linker.rs-2067 */             _ => (),
/* FP:linker.rs-2068 */         }
/* FP:linker.rs-2069 */     }
/* FP:linker.rs-2070 */ 
/* FP:linker.rs-2071 */     fn subsystem(&mut self, _subsystem: &str) {}
/* FP:linker.rs-2072 */ 
/* FP:linker.rs-2073 */     fn linker_plugin_lto(&mut self) {}
/* FP:linker.rs-2074 */ }
/* FP:linker.rs-2075 */ 
/* FP:linker.rs-2076 */ struct BpfLinker<'a> {
/* FP:linker.rs-2077 */     cmd: Command,
/* FP:linker.rs-2078 */     sess: &'a Session,
/* FP:linker.rs-2079 */ }
/* FP:linker.rs-2080 */ 
/* FP:linker.rs-2081 */ impl<'a> Linker for BpfLinker<'a> {
/* FP:linker.rs-2082 */     fn cmd(&mut self) -> &mut Command {
/* FP:linker.rs-2083 */         &mut self.cmd
/* FP:linker.rs-2084 */     }
/* FP:linker.rs-2085 */ 
/* FP:linker.rs-2086 */     fn set_output_kind(
/* FP:linker.rs-2087 */         &mut self,
/* FP:linker.rs-2088 */         _output_kind: LinkOutputKind,
/* FP:linker.rs-2089 */         _crate_type: CrateType,
/* FP:linker.rs-2090 */         _out_filename: &Path,
/* FP:linker.rs-2091 */     ) {
/* FP:linker.rs-2092 */     }
/* FP:linker.rs-2093 */ 
/* FP:linker.rs-2094 */     fn link_staticlib_by_name(&mut self, _name: &str, _verbatim: bool, _whole_archive: bool) {
/* FP:linker.rs-2095 */         panic!("staticlibs not supported")
/* FP:linker.rs-2096 */     }
/* FP:linker.rs-2097 */ 
/* FP:linker.rs-2098 */     fn link_staticlib_by_path(&mut self, path: &Path, _whole_archive: bool) {
/* FP:linker.rs-2099 */         self.link_or_cc_arg(path);
/* FP:linker.rs-2100 */     }
/* FP:linker.rs-2101 */ 
/* FP:linker.rs-2102 */     fn debuginfo(&mut self, _strip: Strip, _: &[PathBuf]) {
/* FP:linker.rs-2103 */         self.link_arg("--debug");
/* FP:linker.rs-2104 */     }
/* FP:linker.rs-2105 */ 
/* FP:linker.rs-2106 */     fn optimize(&mut self) {
/* FP:linker.rs-2107 */         self.link_arg(match self.sess.opts.optimize {
/* FP:linker.rs-2108 */             OptLevel::No => "-O0",
/* FP:linker.rs-2109 */             OptLevel::Less => "-O1",
/* FP:linker.rs-2110 */             OptLevel::More => "-O2",
/* FP:linker.rs-2111 */             OptLevel::Aggressive => "-O3",
/* FP:linker.rs-2112 */             OptLevel::Size => "-Os",
/* FP:linker.rs-2113 */             OptLevel::SizeMin => "-Oz",
/* FP:linker.rs-2114 */         });
/* FP:linker.rs-2115 */     }
/* FP:linker.rs-2116 */ 
/* FP:linker.rs-2117 */     fn full_relro(&mut self) {}
/* FP:linker.rs-2118 */ 
/* FP:linker.rs-2119 */     fn partial_relro(&mut self) {}
/* FP:linker.rs-2120 */ 
/* FP:linker.rs-2121 */     fn no_relro(&mut self) {}
/* FP:linker.rs-2122 */ 
/* FP:linker.rs-2123 */     fn gc_sections(&mut self, _keep_metadata: bool) {}
/* FP:linker.rs-2124 */ 
/* FP:linker.rs-2125 */     fn pgo_gen(&mut self) {}
/* FP:linker.rs-2126 */ 
/* FP:linker.rs-2127 */     fn no_crt_objects(&mut self) {}
/* FP:linker.rs-2128 */ 
/* FP:linker.rs-2129 */     fn no_default_libraries(&mut self) {}
/* FP:linker.rs-2130 */ 
/* FP:linker.rs-2131 */     fn control_flow_guard(&mut self) {}
/* FP:linker.rs-2132 */ 
/* FP:linker.rs-2133 */     fn ehcont_guard(&mut self) {}
/* FP:linker.rs-2134 */ 
/* FP:linker.rs-2135 */     fn export_symbols(
/* FP:linker.rs-2136 */         &mut self,
/* FP:linker.rs-2137 */         tmpdir: &Path,
/* FP:linker.rs-2138 */         _crate_type: CrateType,
/* FP:linker.rs-2139 */         symbols: &[(String, SymbolExportKind)],
/* FP:linker.rs-2140 */     ) {
/* FP:linker.rs-2141 */         let path = tmpdir.join("symbols");
/* FP:linker.rs-2142 */         let res: io::Result<()> = try {
/* FP:linker.rs-2143 */             let mut f = File::create_buffered(&path)?;
/* FP:linker.rs-2144 */             for (sym, _) in symbols {
/* FP:linker.rs-2145 */                 writeln!(f, "{sym}")?;
/* FP:linker.rs-2146 */             }
/* FP:linker.rs-2147 */         };
/* FP:linker.rs-2148 */         if let Err(error) = res {
/* FP:linker.rs-2149 */             self.sess.dcx().emit_fatal(errors::SymbolFileWriteFailure { error });
/* FP:linker.rs-2150 */         } else {
/* FP:linker.rs-2151 */             self.link_arg("--export-symbols").link_arg(&path);
/* FP:linker.rs-2152 */         }
/* FP:linker.rs-2153 */     }
/* FP:linker.rs-2154 */ 
/* FP:linker.rs-2155 */     fn subsystem(&mut self, _subsystem: &str) {}
/* FP:linker.rs-2156 */ 
/* FP:linker.rs-2157 */     fn linker_plugin_lto(&mut self) {}
/* FP:linker.rs-2158 */ }