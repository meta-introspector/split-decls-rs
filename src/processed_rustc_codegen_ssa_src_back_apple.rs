/* FP:apple.rs-0001 */ use std::ffi::OsString;
/* FP:apple.rs-0002 */ use std::path::PathBuf;
/* FP:apple.rs-0003 */ use std::process::Command;
/* FP:apple.rs-0004 */ 
/* FP:apple.rs-0005 */ use itertools::Itertools;
/* FP:apple.rs-0006 */ use crate::rustc_complete::middle::exported_symbols::SymbolExportKind;
/* FP:apple.rs-0007 */ use crate::rustc_complete::Session;
/* FP:apple.rs-0008 */ use crate::rustc_target::spec::Target;
/* FP:apple.rs-0009 */ pub(super) use crate::rustc_target::spec::apple::OSVersion;
/* FP:apple.rs-0010 */ use tracing::debug;
/* FP:apple.rs-0011 */ 
/* FP:apple.rs-0012 */ use crate::errors::{XcrunError, XcrunSdkPathWarning};
/* FP:apple.rs-0013 */ use crate::fluent_generated as fluent;
/* FP:apple.rs-0014 */ 
/* FP:apple.rs-0015 */ #[cfg(test)]
/* FP:apple.rs-0017 */ 
/* FP:apple.rs-0018 */ /// The canonical name of the desired SDK for a given target.
/* FP:apple.rs-0019 */ pub(super) fn sdk_name(target: &Target) -> &'static str {
/* FP:apple.rs-0020 */     match (&*target.os, &*target.env) {
/* FP:apple.rs-0021 */         ("macos", "") => "MacOSX",
/* FP:apple.rs-0022 */         ("ios", "") => "iPhoneOS",
/* FP:apple.rs-0023 */         ("ios", "sim") => "iPhoneSimulator",
/* FP:apple.rs-0024 */         // Mac Catalyst uses the macOS SDK
/* FP:apple.rs-0025 */         ("ios", "macabi") => "MacOSX",
/* FP:apple.rs-0026 */         ("tvos", "") => "AppleTVOS",
/* FP:apple.rs-0027 */         ("tvos", "sim") => "AppleTVSimulator",
/* FP:apple.rs-0028 */         ("visionos", "") => "XROS",
/* FP:apple.rs-0029 */         ("visionos", "sim") => "XRSimulator",
/* FP:apple.rs-0030 */         ("watchos", "") => "WatchOS",
/* FP:apple.rs-0031 */         ("watchos", "sim") => "WatchSimulator",
/* FP:apple.rs-0032 */         (os, abi) => unreachable!("invalid os '{os}' / abi '{abi}' combination for Apple target"),
/* FP:apple.rs-0033 */     }
/* FP:apple.rs-0034 */ }
/* FP:apple.rs-0035 */ 
/* FP:apple.rs-0036 */ pub(super) fn macho_platform(target: &Target) -> u32 {
/* FP:apple.rs-0037 */     match (&*target.os, &*target.env) {
/* FP:apple.rs-0038 */         ("macos", _) => object::macho::PLATFORM_MACOS,
/* FP:apple.rs-0039 */         ("ios", "macabi") => object::macho::PLATFORM_MACCATALYST,
/* FP:apple.rs-0040 */         ("ios", "sim") => object::macho::PLATFORM_IOSSIMULATOR,
/* FP:apple.rs-0041 */         ("ios", _) => object::macho::PLATFORM_IOS,
/* FP:apple.rs-0042 */         ("watchos", "sim") => object::macho::PLATFORM_WATCHOSSIMULATOR,
/* FP:apple.rs-0043 */         ("watchos", _) => object::macho::PLATFORM_WATCHOS,
/* FP:apple.rs-0044 */         ("tvos", "sim") => object::macho::PLATFORM_TVOSSIMULATOR,
/* FP:apple.rs-0045 */         ("tvos", _) => object::macho::PLATFORM_TVOS,
/* FP:apple.rs-0046 */         ("visionos", "sim") => object::macho::PLATFORM_XROSSIMULATOR,
/* FP:apple.rs-0047 */         ("visionos", _) => object::macho::PLATFORM_XROS,
/* FP:apple.rs-0048 */         _ => unreachable!("tried to get Mach-O platform for non-Apple target"),
/* FP:apple.rs-0049 */     }
/* FP:apple.rs-0050 */ }
/* FP:apple.rs-0051 */ 
/* FP:apple.rs-0052 */ /// Add relocation and section data needed for a symbol to be considered
/* FP:apple.rs-0053 */ /// undefined by ld64.
/* FP:apple.rs-0054 */ ///
/* FP:apple.rs-0055 */ /// The relocation must be valid, and hence must point to a valid piece of
/* FP:apple.rs-0056 */ /// machine code, and hence this is unfortunately very architecture-specific.
/* FP:apple.rs-0057 */ ///
/* FP:apple.rs-0058 */ ///
/* FP:apple.rs-0059 */ /// # New architectures
/* FP:apple.rs-0060 */ ///
/* FP:apple.rs-0061 */ /// The values here are basically the same as emitted by the following program:
/* FP:apple.rs-0062 */ ///
/* FP:apple.rs-0063 */ /// ```c
/* FP:apple.rs-0064 */ /// // clang -c foo.c -target $CLANG_TARGET
/* FP:apple.rs-0065 */ /// void foo(void);
/* FP:apple.rs-0066 */ ///
/* FP:apple.rs-0067 */ /// extern int bar;
/* FP:apple.rs-0068 */ ///
/* FP:apple.rs-0069 */ /// void* foobar[2] = {
/* FP:apple.rs-0070 */ ///     (void*)foo,
/* FP:apple.rs-0071 */ ///     (void*)&bar,
/* FP:apple.rs-0072 */ ///     // ...
/* FP:apple.rs-0073 */ /// };
/* FP:apple.rs-0074 */ /// ```
/* FP:apple.rs-0075 */ ///
/* FP:apple.rs-0076 */ /// Can be inspected with:
/* FP:apple.rs-0077 */ /// ```console
/* FP:apple.rs-0078 */ /// objdump --macho --reloc foo.o
/* FP:apple.rs-0079 */ /// objdump --macho --full-contents foo.o
/* FP:apple.rs-0080 */ /// ```
/* FP:apple.rs-0081 */ pub(super) fn add_data_and_relocation(
/* FP:apple.rs-0082 */     file: &mut object::write::Object<'_>,
/* FP:apple.rs-0083 */     section: object::write::SectionId,
/* FP:apple.rs-0084 */     symbol: object::write::SymbolId,
/* FP:apple.rs-0085 */     target: &Target,
/* FP:apple.rs-0086 */     kind: SymbolExportKind,
/* FP:apple.rs-0087 */ ) -> object::write::Result<()> {
/* FP:apple.rs-0088 */     let authenticated_pointer =
/* FP:apple.rs-0089 */         kind == SymbolExportKind::Text && target.llvm_target.starts_with("arm64e");
/* FP:apple.rs-0090 */ 
/* FP:apple.rs-0091 */     let data: &[u8] = match target.pointer_width {
/* FP:apple.rs-0092 */         _ if authenticated_pointer => &[0, 0, 0, 0, 0, 0, 0, 0x80],
/* FP:apple.rs-0093 */         32 => &[0; 4],
/* FP:apple.rs-0094 */         64 => &[0; 8],
/* FP:apple.rs-0095 */         pointer_width => unimplemented!("unsupported Apple pointer width {pointer_width:?}"),
/* FP:apple.rs-0096 */     };
/* FP:apple.rs-0097 */ 
/* FP:apple.rs-0098 */     if target.arch == "x86_64" {
/* FP:apple.rs-0099 */         // Force alignment for the entire section to be 16 on x86_64.
/* FP:apple.rs-0100 */         file.section_mut(section).append_data(&[], 16);
/* FP:apple.rs-0101 */     } else {
/* FP:apple.rs-0102 */         // Elsewhere, the section alignment is the same as the pointer width.
/* FP:apple.rs-0103 */         file.section_mut(section).append_data(&[], target.pointer_width as u64);
/* FP:apple.rs-0104 */     }
/* FP:apple.rs-0105 */ 
/* FP:apple.rs-0106 */     let offset = file.section_mut(section).append_data(data, data.len() as u64);
/* FP:apple.rs-0107 */ 
/* FP:apple.rs-0108 */     let flags = if authenticated_pointer {
/* FP:apple.rs-0109 */         object::write::RelocationFlags::MachO {
/* FP:apple.rs-0110 */             r_type: object::macho::ARM64_RELOC_AUTHENTICATED_POINTER,
/* FP:apple.rs-0111 */             r_pcrel: false,
/* FP:apple.rs-0112 */             r_length: 3,
/* FP:apple.rs-0113 */         }
/* FP:apple.rs-0114 */     } else if target.arch == "arm" {
/* FP:apple.rs-0115 */         // FIXME(madsmtm): Remove once `object` supports 32-bit ARM relocations:
/* FP:apple.rs-0116 */         // https://github.com/gimli-rs/object/pull/757
/* FP:apple.rs-0117 */         object::write::RelocationFlags::MachO {
/* FP:apple.rs-0118 */             r_type: object::macho::ARM_RELOC_VANILLA,
/* FP:apple.rs-0119 */             r_pcrel: false,
/* FP:apple.rs-0120 */             r_length: 2,
/* FP:apple.rs-0121 */         }
/* FP:apple.rs-0122 */     } else {
/* FP:apple.rs-0123 */         object::write::RelocationFlags::Generic {
/* FP:apple.rs-0124 */             kind: object::RelocationKind::Absolute,
/* FP:apple.rs-0125 */             encoding: object::RelocationEncoding::Generic,
/* FP:apple.rs-0126 */             size: target.pointer_width as u8,
/* FP:apple.rs-0127 */         }
/* FP:apple.rs-0128 */     };
/* FP:apple.rs-0129 */ 
/* FP:apple.rs-0130 */     file.add_relocation(section, object::write::Relocation { offset, addend: 0, symbol, flags })?;
/* FP:apple.rs-0131 */ 
/* FP:apple.rs-0132 */     Ok(())
/* FP:apple.rs-0133 */ }
/* FP:apple.rs-0134 */ 
/* FP:apple.rs-0135 */ pub(super) fn add_version_to_llvm_target(
/* FP:apple.rs-0136 */     llvm_target: &str,
/* FP:apple.rs-0137 */     deployment_target: OSVersion,
/* FP:apple.rs-0138 */ ) -> String {
/* FP:apple.rs-0139 */     let mut components = llvm_target.split("-");
/* FP:apple.rs-0140 */     let arch = components.next().expect("apple target should have arch");
/* FP:apple.rs-0141 */     let vendor = components.next().expect("apple target should have vendor");
/* FP:apple.rs-0142 */     let os = components.next().expect("apple target should have os");
/* FP:apple.rs-0143 */     let environment = components.next();
/* FP:apple.rs-0144 */     assert_eq!(components.next(), None, "too many LLVM triple components");
/* FP:apple.rs-0145 */ 
/* FP:apple.rs-0146 */     assert!(
/* FP:apple.rs-0147 */         !os.contains(|c: char| c.is_ascii_digit()),
/* FP:apple.rs-0148 */         "LLVM target must not already be versioned"
/* FP:apple.rs-0149 */     );
/* FP:apple.rs-0150 */ 
/* FP:apple.rs-0151 */     let version = deployment_target.fmt_full();
/* FP:apple.rs-0152 */     if let Some(env) = environment {
/* FP:apple.rs-0153 */         // Insert version into OS, before environment
/* FP:apple.rs-0154 */         format!("{arch}-{vendor}-{os}{version}-{env}")
/* FP:apple.rs-0155 */     } else {
/* FP:apple.rs-0156 */         format!("{arch}-{vendor}-{os}{version}")
/* FP:apple.rs-0157 */     }
/* FP:apple.rs-0158 */ }
/* FP:apple.rs-0159 */ 
/* FP:apple.rs-0160 */ pub(super) fn get_sdk_root(sess: &Session) -> Option<PathBuf> {
/* FP:apple.rs-0161 */     let sdk_name = sdk_name(&sess.target);
/* FP:apple.rs-0162 */ 
/* FP:apple.rs-0163 */     // Attempt to invoke `xcrun` to find the SDK.
/* FP:apple.rs-0164 */     //
/* FP:apple.rs-0165 */     // Note that when cross-compiling from e.g. Linux, the `xcrun` binary may sometimes be provided
/* FP:apple.rs-0166 */     // as a shim by a cross-compilation helper tool. It usually isn't, but we still try nonetheless.
/* FP:apple.rs-0167 */     match xcrun_show_sdk_path(sdk_name, false) {
/* FP:apple.rs-0168 */         Ok((path, stderr)) => {
/* FP:apple.rs-0169 */             // Emit extra stderr, such as if `-verbose` was passed, or if `xcrun` emitted a warning.
/* FP:apple.rs-0170 */             if !stderr.is_empty() {
/* FP:apple.rs-0171 */                 sess.dcx().emit_warn(XcrunSdkPathWarning { sdk_name, stderr });
/* FP:apple.rs-0172 */             }
/* FP:apple.rs-0173 */             Some(path)
/* FP:apple.rs-0174 */         }
/* FP:apple.rs-0175 */         Err(err) => {
/* FP:apple.rs-0176 */             // Failure to find the SDK is not a hard error, since the user might have specified it
/* FP:apple.rs-0177 */             // in a manner unknown to us (moreso if cross-compiling):
/* FP:apple.rs-0178 */             // - A compiler driver like `zig cc` which links using an internally bundled SDK.
/* FP:apple.rs-0179 */             // - Extra linker arguments (`-Clink-arg=-syslibroot`).
/* FP:apple.rs-0180 */             // - A custom linker or custom compiler driver.
/* FP:apple.rs-0181 */             //
/* FP:apple.rs-0182 */             // Though we still warn, since such cases are uncommon, and it is very hard to debug if
/* FP:apple.rs-0183 */             // you do not know the details.
/* FP:apple.rs-0184 */             //
/* FP:apple.rs-0185 */             // FIXME(madsmtm): Make this a lint, to allow deny warnings to work.
/* FP:apple.rs-0186 */             // (Or fix <https://github.com/rust-lang/rust/issues/21204>).
/* FP:apple.rs-0187 */             let mut diag = sess.dcx().create_warn(err);
/* FP:apple.rs-0188 */             diag.note(fluent::codegen_ssa_xcrun_about);
/* FP:apple.rs-0189 */ 
/* FP:apple.rs-0190 */             // Recognize common error cases, and give more Rust-specific error messages for those.
/* FP:apple.rs-0191 */             if let Some(developer_dir) = xcode_select_developer_dir() {
/* FP:apple.rs-0192 */                 diag.arg("developer_dir", &developer_dir);
/* FP:apple.rs-0193 */                 diag.note(fluent::codegen_ssa_xcrun_found_developer_dir);
/* FP:apple.rs-0194 */                 if developer_dir.as_os_str().to_string_lossy().contains("CommandLineTools") {
/* FP:apple.rs-0195 */                     if sdk_name != "MacOSX" {
/* FP:apple.rs-0196 */                         diag.help(fluent::codegen_ssa_xcrun_command_line_tools_insufficient);
/* FP:apple.rs-0197 */                     }
/* FP:apple.rs-0198 */                 }
/* FP:apple.rs-0199 */             } else {
/* FP:apple.rs-0200 */                 diag.help(fluent::codegen_ssa_xcrun_no_developer_dir);
/* FP:apple.rs-0201 */             }
/* FP:apple.rs-0202 */ 
/* FP:apple.rs-0203 */             diag.emit();
/* FP:apple.rs-0204 */             None
/* FP:apple.rs-0205 */         }
/* FP:apple.rs-0206 */     }
/* FP:apple.rs-0207 */ }
/* FP:apple.rs-0208 */ 
/* FP:apple.rs-0209 */ /// Invoke `xcrun --sdk $sdk_name --show-sdk-path` to get the SDK path.
/* FP:apple.rs-0210 */ ///
/* FP:apple.rs-0211 */ /// The exact logic that `xcrun` uses is unspecified (see `man xcrun` for a few details), and may
/* FP:apple.rs-0212 */ /// change between macOS and Xcode versions, but it roughly boils down to finding the active
/* FP:apple.rs-0213 */ /// developer directory, and then invoking `xcodebuild -sdk $sdk_name -version` to get the SDK
/* FP:apple.rs-0214 */ /// details.
/* FP:apple.rs-0215 */ ///
/* FP:apple.rs-0216 */ /// Finding the developer directory is roughly done by looking at, in order:
/* FP:apple.rs-0217 */ /// - The `DEVELOPER_DIR` environment variable.
/* FP:apple.rs-0218 */ /// - The `/var/db/xcode_select_link` symlink (set by `xcode-select --switch`).
/* FP:apple.rs-0219 */ /// - `/Applications/Xcode.app` (hardcoded fallback path).
/* FP:apple.rs-0220 */ /// - `/Library/Developer/CommandLineTools` (hardcoded fallback path).
/* FP:apple.rs-0221 */ ///
/* FP:apple.rs-0222 */ /// Note that `xcrun` caches its result, but with a cold cache this whole operation can be quite
/* FP:apple.rs-0223 */ /// slow, especially so the first time it's run after a reboot.
/* FP:apple.rs-0224 */ fn xcrun_show_sdk_path(
/* FP:apple.rs-0225 */     sdk_name: &'static str,
/* FP:apple.rs-0226 */     verbose: bool,
/* FP:apple.rs-0227 */ ) -> Result<(PathBuf, String), XcrunError> {
/* FP:apple.rs-0228 */     // Intentionally invoke the `xcrun` in PATH, since e.g. nixpkgs provide an `xcrun` shim, so we
/* FP:apple.rs-0229 */     // don't want to require `/usr/bin/xcrun`.
/* FP:apple.rs-0230 */     let mut cmd = Command::new("xcrun");
/* FP:apple.rs-0231 */     if verbose {
/* FP:apple.rs-0232 */         cmd.arg("--verbose");
/* FP:apple.rs-0233 */     }
/* FP:apple.rs-0234 */     // The `--sdk` parameter is the same as in xcodebuild, namely either an absolute path to an SDK,
/* FP:apple.rs-0235 */     // or the (lowercase) canonical name of an SDK.
/* FP:apple.rs-0236 */     cmd.arg("--sdk");
/* FP:apple.rs-0237 */     cmd.arg(&sdk_name.to_lowercase());
/* FP:apple.rs-0238 */     cmd.arg("--show-sdk-path");
/* FP:apple.rs-0239 */ 
/* FP:apple.rs-0240 */     // We do not stream stdout/stderr lines directly to the user, since whether they are warnings or
/* FP:apple.rs-0241 */     // errors depends on the status code at the end.
/* FP:apple.rs-0242 */     let output = cmd.output().map_err(|error| XcrunError::FailedInvoking {
/* FP:apple.rs-0243 */         sdk_name,
/* FP:apple.rs-0244 */         command_formatted: format!("{cmd:?}"),
/* FP:apple.rs-0245 */         error,
/* FP:apple.rs-0246 */     })?;
/* FP:apple.rs-0247 */ 
/* FP:apple.rs-0248 */     // It is fine to do lossy conversion here, non-UTF-8 paths are quite rare on macOS nowadays
/* FP:apple.rs-0249 */     // (only possible with the HFS+ file system), and we only use it for error messages.
/* FP:apple.rs-0250 */     let stderr = String::from_utf8_lossy_owned(output.stderr);
/* FP:apple.rs-0251 */     if !stderr.is_empty() {
/* FP:apple.rs-0252 */         debug!(stderr, "original xcrun stderr");
/* FP:apple.rs-0253 */     }
/* FP:apple.rs-0254 */ 
/* FP:apple.rs-0255 */     // Some versions of `xcodebuild` output beefy errors when invoked via `xcrun`,
/* FP:apple.rs-0256 */     // but these are usually red herrings.
/* FP:apple.rs-0257 */     let stderr = stderr
/* FP:apple.rs-0258 */         .lines()
/* FP:apple.rs-0259 */         .filter(|line| {
/* FP:apple.rs-0260 */             !line.contains("Writing error result bundle")
/* FP:apple.rs-0261 */                 && !line.contains("Requested but did not find extension point with identifier")
/* FP:apple.rs-0262 */         })
/* FP:apple.rs-0263 */         .join("\n");
/* FP:apple.rs-0264 */ 
/* FP:apple.rs-0265 */     if output.status.success() {
/* FP:apple.rs-0266 */         Ok((stdout_to_path(output.stdout), stderr))
/* FP:apple.rs-0267 */     } else {
/* FP:apple.rs-0268 */         // Output both stdout and stderr, since shims of `xcrun` (such as the one provided by
/* FP:apple.rs-0269 */         // nixpkgs), do not always use stderr for errors.
/* FP:apple.rs-0270 */         let stdout = String::from_utf8_lossy_owned(output.stdout).trim().to_string();
/* FP:apple.rs-0271 */         Err(XcrunError::Unsuccessful {
/* FP:apple.rs-0272 */             sdk_name,
/* FP:apple.rs-0273 */             command_formatted: format!("{cmd:?}"),
/* FP:apple.rs-0274 */             stdout,
/* FP:apple.rs-0275 */             stderr,
/* FP:apple.rs-0276 */         })
/* FP:apple.rs-0277 */     }
/* FP:apple.rs-0278 */ }
/* FP:apple.rs-0279 */ 
/* FP:apple.rs-0280 */ /// Invoke `xcode-select --print-path`, and return the current developer directory.
/* FP:apple.rs-0281 */ ///
/* FP:apple.rs-0282 */ /// NOTE: We don't do any error handling here, this is only used as a canary in diagnostics (`xcrun`
/* FP:apple.rs-0283 */ /// will have already emitted the relevant error information).
/* FP:apple.rs-0284 */ fn xcode_select_developer_dir() -> Option<PathBuf> {
/* FP:apple.rs-0285 */     let mut cmd = Command::new("xcode-select");
/* FP:apple.rs-0286 */     cmd.arg("--print-path");
/* FP:apple.rs-0287 */     let output = cmd.output().ok()?;
/* FP:apple.rs-0288 */     if !output.status.success() {
/* FP:apple.rs-0289 */         return None;
/* FP:apple.rs-0290 */     }
/* FP:apple.rs-0291 */     Some(stdout_to_path(output.stdout))
/* FP:apple.rs-0292 */ }
/* FP:apple.rs-0293 */ 
/* FP:apple.rs-0294 */ fn stdout_to_path(mut stdout: Vec<u8>) -> PathBuf {
/* FP:apple.rs-0295 */     // Remove trailing newline.
/* FP:apple.rs-0296 */     if let Some(b'\n') = stdout.last() {
/* FP:apple.rs-0297 */         let _ = stdout.pop().unwrap();
/* FP:apple.rs-0298 */     }
/* FP:apple.rs-0299 */     #[cfg(unix)]
/* FP:apple.rs-0300 */     let path = <OsString as std::os::unix::ffi::OsStringExt>::from_vec(stdout);
/* FP:apple.rs-0301 */     #[cfg(not(unix))] // Not so important, this is mostly used on macOS
/* FP:apple.rs-0302 */     let path = OsString::from(String::from_utf8(stdout).expect("stdout must be UTF-8"));
/* FP:apple.rs-0303 */     PathBuf::from(path)
/* FP:apple.rs-0304 */ }