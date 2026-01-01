/* FP:config.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_session_src_config_UNPARSEABLE_0001
/* FP:config.rs-0002 */ // Contains infrastructure for configuring the compiler, including parsing
/* FP:config.rs-0003 */ // command-line options.
/* FP:config.rs-0004 */ 
/* FP:config.rs-0005 */ #[allow(rustc::untranslatable_diagnostic)] // FIXME: make this translatable
/* FP:config.rs-0006 */ 
/* FP:config.rs-0007 */ use std::collections::btree_map::{
/* FP:config.rs-0008 */     Iter as BTreeMapIter, Keys as BTreeMapKeysIter, Values as BTreeMapValuesIter,
/* FP:config.rs-0009 */ };
/* FP:config.rs-0010 */ use std::collections::{BTreeMap, BTreeSet};
/* FP:config.rs-0011 */ use std::ffi::OsStr;
/* FP:config.rs-0012 */ use std::hash::Hash;
/* FP:config.rs-0013 */ use std::path::{Path, PathBuf};
/* FP:config.rs-0014 */ use std::str::{self, FromStr};
/* FP:config.rs-0015 */ use std::sync::LazyLock;
/* FP:config.rs-0016 */ use std::{cmp, fmt, fs, iter};
/* FP:config.rs-0017 */ 
/* FP:config.rs-0018 */ use externs::{ExternOpt, split_extern_opt};
/* FP:config.rs-0019 */ use crate::rustc_data_structures::fx::{FxHashSet, FxIndexMap};
/* FP:config.rs-0020 */ use crate::rustc_data_structures::stable_hasher::{StableHasher, StableOrd, ToStableHashKey};
/* FP:config.rs-0021 */ use crate::rustc_complete::emitter::HumanReadableErrorType;
/* FP:config.rs-0022 */ use crate::rustc_complete::{ColorConfig, DiagArgValue, DiagCtxtFlags, IntoDiagArg};
/* FP:config.rs-0023 */ use crate::rustc_feature::UnstableFeatures;
/* FP:config.rs-0024 */ use rustc_hashes::Hash64;
/* FP:config.rs-0025 */ use rustc_macros::{Decodable, Encodable, HashStable_Generic};
/* FP:config.rs-0026 */ use crate::rustc_complete::edition::{DEFAULT_EDITION, EDITION_NAME_LIST, Edition, LATEST_STABLE_EDITION};
/* FP:config.rs-0027 */ use crate::rustc_complete::source_map::FilePathMapping;
/* FP:config.rs-0028 */ use crate::rustc_complete::{
/* FP:config.rs-0029 */     FileName, FileNameDisplayPreference, FileNameEmbeddablePreference, RealFileName,
/* FP:config.rs-0030 */     SourceFileHashAlgorithm, Symbol, sym,
/* FP:config.rs-0031 */ };
/* FP:config.rs-0032 */ use crate::rustc_target::spec::{
/* FP:config.rs-0033 */     FramePointer, LinkSelfContainedComponents, LinkerFeatures, SplitDebuginfo, Target, TargetTuple,
/* FP:config.rs-0034 */ };
/* FP:config.rs-0035 */ use tracing::debug;
/* FP:config.rs-0036 */ 
/* FP:config.rs-0037 */ pub use crate::config::cfg::{Cfg, CheckCfg, ExpectedValues};
/* FP:config.rs-0038 */ use crate::config::native_libs::parse_native_libs;
/* FP:config.rs-0039 */ use crate::errors::FileWriteFail;
/* FP:config.rs-0040 */ pub use crate::options::*;
/* FP:config.rs-0041 */ use crate::search_paths::SearchPath;
/* FP:config.rs-0042 */ use crate::utils::CanonicalizedPath;
/* FP:config.rs-0043 */ use crate::{EarlyDiagCtxt, HashStableContext, Session, filesearch, lint};
/* FP:config.rs-0044 */ 
/* FP:config.rs-0049 */ 
/* FP:config.rs-0050 */ pub const PRINT_KINDS: &[(&str, PrintKind)] = &[
/* FP:config.rs-0051 */     // tidy-alphabetical-start
/* FP:config.rs-0052 */     ("all-target-specs-json", PrintKind::AllTargetSpecsJson),
/* FP:config.rs-0053 */     ("calling-conventions", PrintKind::CallingConventions),
/* FP:config.rs-0054 */     ("cfg", PrintKind::Cfg),
/* FP:config.rs-0055 */     ("check-cfg", PrintKind::CheckCfg),
/* FP:config.rs-0056 */     ("code-models", PrintKind::CodeModels),
/* FP:config.rs-0057 */     ("crate-name", PrintKind::CrateName),
/* FP:config.rs-0058 */     ("crate-root-lint-levels", PrintKind::CrateRootLintLevels),
/* FP:config.rs-0059 */     ("deployment-target", PrintKind::DeploymentTarget),
/* FP:config.rs-0060 */     ("file-names", PrintKind::FileNames),
/* FP:config.rs-0061 */     ("host-tuple", PrintKind::HostTuple),
/* FP:config.rs-0062 */     ("link-args", PrintKind::LinkArgs),
/* FP:config.rs-0063 */     ("native-static-libs", PrintKind::NativeStaticLibs),
/* FP:config.rs-0064 */     ("relocation-models", PrintKind::RelocationModels),
/* FP:config.rs-0065 */     ("split-debuginfo", PrintKind::SplitDebuginfo),
/* FP:config.rs-0066 */     ("stack-protector-strategies", PrintKind::StackProtectorStrategies),
/* FP:config.rs-0067 */     ("supported-crate-types", PrintKind::SupportedCrateTypes),
/* FP:config.rs-0068 */     ("sysroot", PrintKind::Sysroot),
/* FP:config.rs-0069 */     ("target-cpus", PrintKind::TargetCPUs),
/* FP:config.rs-0070 */     ("target-features", PrintKind::TargetFeatures),
/* FP:config.rs-0071 */     ("target-libdir", PrintKind::TargetLibdir),
/* FP:config.rs-0072 */     ("target-list", PrintKind::TargetList),
/* FP:config.rs-0073 */     ("target-spec-json", PrintKind::TargetSpecJson),
/* FP:config.rs-0074 */     ("target-spec-json-schema", PrintKind::TargetSpecJsonSchema),
/* FP:config.rs-0075 */     ("tls-models", PrintKind::TlsModels),
/* FP:config.rs-0076 */     // tidy-alphabetical-end
/* FP:config.rs-0077 */ ];
/* FP:config.rs-0078 */ 
/* FP:config.rs-0079 */ /// The different settings that the `-C strip` flag can have.
/* FP:config.rs-0080 */ #[derive(Clone, Copy, PartialEq, Hash, Debug)]
/* FP:config.rs-0081 */ pub enum Strip {
/* FP:config.rs-0082 */     /// Do not strip at all.
/* FP:config.rs-0083 */     None,
/* FP:config.rs-0084 */ 
/* FP:config.rs-0085 */     /// Strip debuginfo.
/* FP:config.rs-0086 */     Debuginfo,
/* FP:config.rs-0087 */ 
/* FP:config.rs-0088 */     /// Strip all symbols.
/* FP:config.rs-0089 */     Symbols,
/* FP:config.rs-0090 */ }
/* FP:config.rs-0091 */ 
/* FP:config.rs-0092 */ /// The different settings that the `-C control-flow-guard` flag can have.
/* FP:config.rs-0093 */ #[derive(Clone, Copy, PartialEq, Hash, Debug)]
/* FP:config.rs-0094 */ pub enum CFGuard {
/* FP:config.rs-0095 */     /// Do not emit Control Flow Guard metadata or checks.
/* FP:config.rs-0096 */     Disabled,
/* FP:config.rs-0097 */ 
/* FP:config.rs-0098 */     /// Emit Control Flow Guard metadata but no checks.
/* FP:config.rs-0099 */     NoChecks,
/* FP:config.rs-0100 */ 
/* FP:config.rs-0101 */     /// Emit Control Flow Guard metadata and checks.
/* FP:config.rs-0102 */     Checks,
/* FP:config.rs-0103 */ }
/* FP:config.rs-0104 */ 
/* FP:config.rs-0105 */ /// The different settings that the `-Z cf-protection` flag can have.
/* FP:config.rs-0106 */ #[derive(Clone, Copy, PartialEq, Hash, Debug)]
/* FP:config.rs-0107 */ pub enum CFProtection {
/* FP:config.rs-0108 */     /// Do not enable control-flow protection
/* FP:config.rs-0109 */     None,
/* FP:config.rs-0110 */ 
/* FP:config.rs-0111 */     /// Emit control-flow protection for branches (enables indirect branch tracking).
/* FP:config.rs-0112 */     Branch,
/* FP:config.rs-0113 */ 
/* FP:config.rs-0114 */     /// Emit control-flow protection for returns.
/* FP:config.rs-0115 */     Return,
/* FP:config.rs-0116 */ 
/* FP:config.rs-0117 */     /// Emit control-flow protection for both branches and returns.
/* FP:config.rs-0118 */     Full,
/* FP:config.rs-0119 */ }
/* FP:config.rs-0120 */ 
/* FP:config.rs-0121 */ #[derive(Clone, Copy, Debug, PartialEq, Hash, HashStable_Generic)]
/* FP:config.rs-0122 */ pub enum OptLevel {
/* FP:config.rs-0123 */     /// `-Copt-level=0`
/* FP:config.rs-0124 */     No,
/* FP:config.rs-0125 */     /// `-Copt-level=1`
/* FP:config.rs-0126 */     Less,
/* FP:config.rs-0127 */     /// `-Copt-level=2`
/* FP:config.rs-0128 */     More,
/* FP:config.rs-0129 */     /// `-Copt-level=3` / `-O`
/* FP:config.rs-0130 */     Aggressive,
/* FP:config.rs-0131 */     /// `-Copt-level=s`
/* FP:config.rs-0132 */     Size,
/* FP:config.rs-0133 */     /// `-Copt-level=z`
/* FP:config.rs-0134 */     SizeMin,
/* FP:config.rs-0135 */ }
/* FP:config.rs-0136 */ 
/* FP:config.rs-0137 */ /// This is what the `LtoCli` values get mapped to after resolving defaults and
/* FP:config.rs-0138 */ /// and taking other command line options into account.
/* FP:config.rs-0139 */ ///
/* FP:config.rs-0140 */ /// Note that linker plugin-based LTO is a different mechanism entirely.
/* FP:config.rs-0141 */ #[derive(Clone, PartialEq)]
/* FP:config.rs-0142 */ pub enum Lto {
/* FP:config.rs-0143 */     /// Don't do any LTO whatsoever.
/* FP:config.rs-0144 */     No,
/* FP:config.rs-0145 */ 
/* FP:config.rs-0146 */     /// Do a full-crate-graph (inter-crate) LTO with ThinLTO.
/* FP:config.rs-0147 */     Thin,
/* FP:config.rs-0148 */ 
/* FP:config.rs-0149 */     /// Do a local ThinLTO (intra-crate, over the CodeGen Units of the local crate only). This is
/* FP:config.rs-0150 */     /// only relevant if multiple CGUs are used.
/* FP:config.rs-0151 */     ThinLocal,
/* FP:config.rs-0152 */ 
/* FP:config.rs-0153 */     /// Do a full-crate-graph (inter-crate) LTO with "fat" LTO.
/* FP:config.rs-0154 */     Fat,
/* FP:config.rs-0155 */ }
/* FP:config.rs-0156 */ 
/* FP:config.rs-0157 */ /// The different settings that the `-C lto` flag can have.
/* FP:config.rs-0158 */ #[derive(Clone, Copy, PartialEq, Hash, Debug)]
/* FP:config.rs-0159 */ pub enum LtoCli {
/* FP:config.rs-0160 */     /// `-C lto=no`
/* FP:config.rs-0161 */     No,
/* FP:config.rs-0162 */     /// `-C lto=yes`
/* FP:config.rs-0163 */     Yes,
/* FP:config.rs-0164 */     /// `-C lto`
/* FP:config.rs-0165 */     NoParam,
/* FP:config.rs-0166 */     /// `-C lto=thin`
/* FP:config.rs-0167 */     Thin,
/* FP:config.rs-0168 */     /// `-C lto=fat`
/* FP:config.rs-0169 */     Fat,
/* FP:config.rs-0170 */     /// No `-C lto` flag passed
/* FP:config.rs-0171 */     Unspecified,
/* FP:config.rs-0172 */ }
/* FP:config.rs-0173 */ 
/* FP:config.rs-0174 */ /// The different settings that the `-C instrument-coverage` flag can have.
/* FP:config.rs-0175 */ #[derive(Clone, Copy, PartialEq, Hash, Debug)]
/* FP:config.rs-0176 */ pub enum InstrumentCoverage {
/* FP:config.rs-0177 */     /// `-C instrument-coverage=no` (or `off`, `false` etc.)
/* FP:config.rs-0178 */     No,
/* FP:config.rs-0179 */     /// `-C instrument-coverage` or `-C instrument-coverage=yes`
/* FP:config.rs-0180 */     Yes,
/* FP:config.rs-0181 */ }
/* FP:config.rs-0182 */ 
/* FP:config.rs-0183 */ /// Individual flag values controlled by `-Zcoverage-options`.
/* FP:config.rs-0184 */ #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
/* FP:config.rs-0185 */ pub struct CoverageOptions {
/* FP:config.rs-0186 */     pub level: CoverageLevel,
/* FP:config.rs-0187 */ 
/* FP:config.rs-0188 */     /// **(internal test-only flag)**
/* FP:config.rs-0189 */     /// `-Zcoverage-options=discard-all-spans-in-codegen`: During codegen,
/* FP:config.rs-0190 */     /// discard all coverage spans as though they were invalid. Needed by
/* FP:config.rs-0191 */     /// regression tests for #133606, because we don't have an easy way to
/* FP:config.rs-0192 */     /// reproduce it from actual source code.
/* FP:config.rs-0193 */     pub discard_all_spans_in_codegen: bool,
/* FP:config.rs-0194 */ }
/* FP:config.rs-0195 */ 
/* FP:config.rs-0196 */ /// Controls whether branch coverage is enabled.
/* FP:config.rs-0197 */ #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
/* FP:config.rs-0198 */ pub enum CoverageLevel {
/* FP:config.rs-0199 */     /// Instrument for coverage at the MIR block level.
/* FP:config.rs-0200 */     #[default]
/* FP:config.rs-0201 */     Block,
/* FP:config.rs-0202 */     /// Also instrument branch points (includes block coverage).
/* FP:config.rs-0203 */     Branch,
/* FP:config.rs-0204 */     /// Same as branch coverage, but also adds branch instrumentation for
/* FP:config.rs-0205 */     /// certain boolean expressions that are not directly used for branching.
/* FP:config.rs-0206 */     ///
/* FP:config.rs-0207 */     /// For example, in the following code, `b` does not directly participate
/* FP:config.rs-0208 */     /// in a branch, but condition coverage will instrument it as its own
/* FP:config.rs-0209 */     /// artificial branch:
/* FP:config.rs-0210 */     /// ```
/* FP:config.rs-0211 */     /// # let (a, b) = (false, true);
/* FP:config.rs-0212 */     /// let x = a && b;
/* FP:config.rs-0213 */     /// //           ^ last operand
/* FP:config.rs-0214 */     /// ```
/* FP:config.rs-0215 */     ///
/* FP:config.rs-0216 */     /// This level is mainly intended to be a stepping-stone towards full MC/DC
/* FP:config.rs-0217 */     /// instrumentation, so it might be removed in the future when MC/DC is
/* FP:config.rs-0218 */     /// sufficiently complete, or if it is making MC/DC changes difficult.
/* FP:config.rs-0219 */     Condition,
/* FP:config.rs-0220 */ }
/* FP:config.rs-0221 */ 
/* FP:config.rs-0222 */ // The different settings that the `-Z offload` flag can have.
/* FP:config.rs-0223 */ #[derive(Clone, Copy, PartialEq, Hash, Debug)]
/* FP:config.rs-0224 */ pub enum Offload {
/* FP:config.rs-0225 */     /// Enable the llvm offload pipeline
/* FP:config.rs-0226 */     Enable,
/* FP:config.rs-0227 */ }
/* FP:config.rs-0228 */ 
/* FP:config.rs-0229 */ /// The different settings that the `-Z autodiff` flag can have.
/* FP:config.rs-0230 */ #[derive(Clone, PartialEq, Hash, Debug)]
/* FP:config.rs-0231 */ pub enum AutoDiff {
/* FP:config.rs-0232 */     /// Enable the autodiff opt pipeline
/* FP:config.rs-0233 */     Enable,
/* FP:config.rs-0234 */ 
/* FP:config.rs-0235 */     /// Print TypeAnalysis information
/* FP:config.rs-0236 */     PrintTA,
/* FP:config.rs-0237 */     /// Print TypeAnalysis information for a specific function
/* FP:config.rs-0238 */     PrintTAFn(String),
/* FP:config.rs-0239 */     /// Print ActivityAnalysis Information
/* FP:config.rs-0240 */     PrintAA,
/* FP:config.rs-0241 */     /// Print Performance Warnings from Enzyme
/* FP:config.rs-0242 */     PrintPerf,
/* FP:config.rs-0243 */     /// Print intermediate IR generation steps
/* FP:config.rs-0244 */     PrintSteps,
/* FP:config.rs-0245 */     /// Print the module, before running autodiff.
/* FP:config.rs-0246 */     PrintModBefore,
/* FP:config.rs-0247 */     /// Print the module after running autodiff.
/* FP:config.rs-0248 */     PrintModAfter,
/* FP:config.rs-0249 */     /// Print the module after running autodiff and optimizations.
/* FP:config.rs-0250 */     PrintModFinal,
/* FP:config.rs-0251 */ 
/* FP:config.rs-0252 */     /// Print all passes scheduled by LLVM
/* FP:config.rs-0253 */     PrintPasses,
/* FP:config.rs-0254 */     /// Disable extra opt run after running autodiff
/* FP:config.rs-0255 */     NoPostopt,
/* FP:config.rs-0256 */     /// Enzyme's loose type debug helper (can cause incorrect gradients!!)
/* FP:config.rs-0257 */     /// Usable in cases where Enzyme errors with `can not deduce type of X`.
/* FP:config.rs-0258 */     LooseTypes,
/* FP:config.rs-0259 */     /// Runs Enzyme's aggressive inlining
/* FP:config.rs-0260 */     Inline,
/* FP:config.rs-0261 */ }
/* FP:config.rs-0262 */ 
/* FP:config.rs-0263 */ /// Settings for `-Z instrument-xray` flag.
/* FP:config.rs-0264 */ #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
/* FP:config.rs-0265 */ pub struct InstrumentXRay {
/* FP:config.rs-0266 */     /// `-Z instrument-xray=always`, force instrumentation
/* FP:config.rs-0267 */     pub always: bool,
/* FP:config.rs-0268 */     /// `-Z instrument-xray=never`, disable instrumentation
/* FP:config.rs-0269 */     pub never: bool,
/* FP:config.rs-0270 */     /// `-Z instrument-xray=ignore-loops`, ignore presence of loops,
/* FP:config.rs-0271 */     /// instrument functions based only on instruction count
/* FP:config.rs-0272 */     pub ignore_loops: bool,
/* FP:config.rs-0273 */     /// `-Z instrument-xray=instruction-threshold=N`, explicitly set instruction threshold
/* FP:config.rs-0274 */     /// for instrumentation, or `None` to use compiler's default
/* FP:config.rs-0275 */     pub instruction_threshold: Option<usize>,
/* FP:config.rs-0276 */     /// `-Z instrument-xray=skip-entry`, do not instrument function entry
/* FP:config.rs-0277 */     pub skip_entry: bool,
/* FP:config.rs-0278 */     /// `-Z instrument-xray=skip-exit`, do not instrument function exit
/* FP:config.rs-0279 */     pub skip_exit: bool,
/* FP:config.rs-0280 */ }
/* FP:config.rs-0281 */ 
/* FP:config.rs-0282 */ #[derive(Clone, PartialEq, Hash, Debug)]
/* FP:config.rs-0283 */ pub enum LinkerPluginLto {
/* FP:config.rs-0284 */     LinkerPlugin(PathBuf),
/* FP:config.rs-0285 */     LinkerPluginAuto,
/* FP:config.rs-0286 */     Disabled,
/* FP:config.rs-0287 */ }
/* FP:config.rs-0288 */ 
/* FP:config.rs-0289 */ impl LinkerPluginLto {
/* FP:config.rs-0290 */     pub fn enabled(&self) -> bool {
/* FP:config.rs-0291 */         match *self {
/* FP:config.rs-0292 */             LinkerPluginLto::LinkerPlugin(_) | LinkerPluginLto::LinkerPluginAuto => true,
/* FP:config.rs-0293 */             LinkerPluginLto::Disabled => false,
/* FP:config.rs-0294 */         }
/* FP:config.rs-0295 */     }
/* FP:config.rs-0296 */ }
/* FP:config.rs-0297 */ 
/* FP:config.rs-0298 */ /// The different values `-C link-self-contained` can take: a list of individually enabled or
/* FP:config.rs-0299 */ /// disabled components used during linking, coming from the rustc distribution, instead of being
/* FP:config.rs-0300 */ /// found somewhere on the host system.
/* FP:config.rs-0301 */ ///
/* FP:config.rs-0302 */ /// They can be set in bulk via `-C link-self-contained=yes|y|on` or `-C
/* FP:config.rs-0303 */ /// link-self-contained=no|n|off`, and those boolean values are the historical defaults.
/* FP:config.rs-0304 */ ///
/* FP:config.rs-0305 */ /// But each component is fine-grained, and can be unstably targeted, to use:
/* FP:config.rs-0306 */ /// - some CRT objects
/* FP:config.rs-0307 */ /// - the libc static library
/* FP:config.rs-0308 */ /// - libgcc/libunwind libraries
/* FP:config.rs-0309 */ /// - a linker we distribute
/* FP:config.rs-0310 */ /// - some sanitizer runtime libraries
/* FP:config.rs-0311 */ /// - all other MinGW libraries and Windows import libs
/* FP:config.rs-0312 */ ///
/* FP:config.rs-0313 */ #[derive(Default, Clone, PartialEq, Debug)]
/* FP:config.rs-0314 */ pub struct LinkSelfContained {
/* FP:config.rs-0315 */     /// Whether the user explicitly set `-C link-self-contained` on or off, the historical values.
/* FP:config.rs-0316 */     /// Used for compatibility with the existing opt-in and target inference.
/* FP:config.rs-0317 */     pub explicitly_set: Option<bool>,
/* FP:config.rs-0318 */ 
/* FP:config.rs-0319 */     /// The components that are enabled on the CLI, using the `+component` syntax or one of the
/* FP:config.rs-0320 */     /// `true` shortcuts.
/* FP:config.rs-0321 */     enabled_components: LinkSelfContainedComponents,
/* FP:config.rs-0322 */ 
/* FP:config.rs-0323 */     /// The components that are disabled on the CLI, using the `-component` syntax or one of the
/* FP:config.rs-0324 */     /// `false` shortcuts.
/* FP:config.rs-0325 */     disabled_components: LinkSelfContainedComponents,
/* FP:config.rs-0326 */ }
/* FP:config.rs-0327 */ 
/* FP:config.rs-0328 */ impl LinkSelfContained {
/* FP:config.rs-0329 */     /// Incorporates an enabled or disabled component as specified on the CLI, if possible.
/* FP:config.rs-0330 */     /// For example: `+linker`, and `-crto`.
/* FP:config.rs-0331 */     pub(crate) fn handle_cli_component(&mut self, component: &str) -> Option<()> {
/* FP:config.rs-0332 */         // Note that for example `-Cself-contained=y -Cself-contained=-linker` is not an explicit
/* FP:config.rs-0333 */         // set of all values like `y` or `n` used to be. Therefore, if this flag had previously been
/* FP:config.rs-0334 */         // set in bulk with its historical values, then manually setting a component clears that
/* FP:config.rs-0335 */         // `explicitly_set` state.
/* FP:config.rs-0336 */         if let Some(component_to_enable) = component.strip_prefix('+') {
/* FP:config.rs-0337 */             self.explicitly_set = None;
/* FP:config.rs-0338 */             self.enabled_components
/* FP:config.rs-0339 */                 .insert(LinkSelfContainedComponents::from_str(component_to_enable).ok()?);
/* FP:config.rs-0340 */             Some(())
/* FP:config.rs-0341 */         } else if let Some(component_to_disable) = component.strip_prefix('-') {
/* FP:config.rs-0342 */             self.explicitly_set = None;
/* FP:config.rs-0343 */             self.disabled_components
/* FP:config.rs-0344 */                 .insert(LinkSelfContainedComponents::from_str(component_to_disable).ok()?);
/* FP:config.rs-0345 */             Some(())
/* FP:config.rs-0346 */         } else {
/* FP:config.rs-0347 */             None
/* FP:config.rs-0348 */         }
/* FP:config.rs-0349 */     }
/* FP:config.rs-0350 */ 
/* FP:config.rs-0351 */     /// Turns all components on or off and records that this was done explicitly for compatibility
/* FP:config.rs-0352 */     /// purposes.
/* FP:config.rs-0353 */     pub(crate) fn set_all_explicitly(&mut self, enabled: bool) {
/* FP:config.rs-0354 */         self.explicitly_set = Some(enabled);
/* FP:config.rs-0355 */ 
/* FP:config.rs-0356 */         if enabled {
/* FP:config.rs-0357 */             self.enabled_components = LinkSelfContainedComponents::all();
/* FP:config.rs-0358 */             self.disabled_components = LinkSelfContainedComponents::empty();
/* FP:config.rs-0359 */         } else {
/* FP:config.rs-0360 */             self.enabled_components = LinkSelfContainedComponents::empty();
/* FP:config.rs-0361 */             self.disabled_components = LinkSelfContainedComponents::all();
/* FP:config.rs-0362 */         }
/* FP:config.rs-0363 */     }
/* FP:config.rs-0364 */ 
/* FP:config.rs-0365 */     /// Helper creating a fully enabled `LinkSelfContained` instance. Used in tests.
/* FP:config.rs-0366 */     pub fn on() -> Self {
/* FP:config.rs-0367 */         let mut on = LinkSelfContained::default();
/* FP:config.rs-0368 */         on.set_all_explicitly(true);
/* FP:config.rs-0369 */         on
/* FP:config.rs-0370 */     }
/* FP:config.rs-0371 */ 
/* FP:config.rs-0372 */     /// To help checking CLI usage while some of the values are unstable: returns whether one of the
/* FP:config.rs-0373 */     /// unstable components was set individually, for the given `TargetTuple`. This would also
/* FP:config.rs-0374 */     /// require the `-Zunstable-options` flag, to be allowed.
/* FP:config.rs-0375 */     fn check_unstable_variants(&self, target_tuple: &TargetTuple) -> Result<(), String> {
/* FP:config.rs-0376 */         if self.explicitly_set.is_some() {
/* FP:config.rs-0377 */             return Ok(());
/* FP:config.rs-0378 */         }
/* FP:config.rs-0379 */ 
/* FP:config.rs-0380 */         // `-C link-self-contained=-linker` is only stable on x64 linux.
/* FP:config.rs-0381 */         let has_minus_linker = self.disabled_components.is_linker_enabled();
/* FP:config.rs-0382 */         if has_minus_linker && target_tuple.tuple() != "x86_64-unknown-linux-gnu" {
/* FP:config.rs-0383 */             return Err(format!(
/* FP:config.rs-0384 */                 "`-C link-self-contained=-linker` is unstable on the `{target_tuple}` \
/* FP:config.rs-0385 */                     target. The `-Z unstable-options` flag must also be passed to use it on this target",
/* FP:config.rs-0386 */             ));
/* FP:config.rs-0387 */         }
/* FP:config.rs-0388 */ 
/* FP:config.rs-0389 */         // Any `+linker` or other component used is unstable, and that's an error.
/* FP:config.rs-0390 */         let unstable_enabled = self.enabled_components;
/* FP:config.rs-0391 */         let unstable_disabled = self.disabled_components - LinkSelfContainedComponents::LINKER;
/* FP:config.rs-0392 */         if !unstable_enabled.union(unstable_disabled).is_empty() {
/* FP:config.rs-0393 */             return Err(String::from(
/* FP:config.rs-0394 */                 "only `-C link-self-contained` values `y`/`yes`/`on`/`n`/`no`/`off`/`-linker` \
/* FP:config.rs-0395 */                 are stable, the `-Z unstable-options` flag must also be passed to use \
/* FP:config.rs-0396 */                 the unstable values",
/* FP:config.rs-0397 */             ));
/* FP:config.rs-0398 */         }
/* FP:config.rs-0399 */ 
/* FP:config.rs-0400 */         Ok(())
/* FP:config.rs-0401 */     }
/* FP:config.rs-0402 */ 
/* FP:config.rs-0403 */     /// Returns whether the self-contained linker component was enabled on the CLI, using the
/* FP:config.rs-0404 */     /// `-C link-self-contained=+linker` syntax, or one of the `true` shortcuts.
/* FP:config.rs-0405 */     pub fn is_linker_enabled(&self) -> bool {
/* FP:config.rs-0406 */         self.enabled_components.contains(LinkSelfContainedComponents::LINKER)
/* FP:config.rs-0407 */     }
/* FP:config.rs-0408 */ 
/* FP:config.rs-0409 */     /// Returns whether the self-contained linker component was disabled on the CLI, using the
/* FP:config.rs-0410 */     /// `-C link-self-contained=-linker` syntax, or one of the `false` shortcuts.
/* FP:config.rs-0411 */     pub fn is_linker_disabled(&self) -> bool {
/* FP:config.rs-0412 */         self.disabled_components.contains(LinkSelfContainedComponents::LINKER)
/* FP:config.rs-0413 */     }
/* FP:config.rs-0414 */ 
/* FP:config.rs-0415 */     /// Returns CLI inconsistencies to emit errors: individual components were both enabled and
/* FP:config.rs-0416 */     /// disabled.
/* FP:config.rs-0417 */     fn check_consistency(&self) -> Option<LinkSelfContainedComponents> {
/* FP:config.rs-0418 */         if self.explicitly_set.is_some() {
/* FP:config.rs-0419 */             None
/* FP:config.rs-0420 */         } else {
/* FP:config.rs-0421 */             let common = self.enabled_components.intersection(self.disabled_components);
/* FP:config.rs-0422 */             if common.is_empty() { None } else { Some(common) }
/* FP:config.rs-0423 */         }
/* FP:config.rs-0424 */     }
/* FP:config.rs-0425 */ }
/* FP:config.rs-0426 */ 
/* FP:config.rs-0427 */ /// The different values that `-C linker-features` can take on the CLI: a list of individually
/* FP:config.rs-0428 */ /// enabled or disabled features used during linking.
/* FP:config.rs-0429 */ ///
/* FP:config.rs-0430 */ /// There is no need to enable or disable them in bulk. Each feature is fine-grained, and can be
/* FP:config.rs-0431 */ /// used to turn `LinkerFeatures` on or off, without needing to change the linker flavor:
/* FP:config.rs-0432 */ /// - using the system lld, or the self-contained `rust-lld` linker
/* FP:config.rs-0433 */ /// - using a C/C++ compiler to drive the linker (not yet exposed on the CLI)
/* FP:config.rs-0434 */ /// - etc.
/* FP:config.rs-0435 */ #[derive(Default, Copy, Clone, PartialEq, Debug)]
/* FP:config.rs-0436 */ pub struct LinkerFeaturesCli {
/* FP:config.rs-0437 */     /// The linker features that are enabled on the CLI, using the `+feature` syntax.
/* FP:config.rs-0438 */     pub enabled: LinkerFeatures,
/* FP:config.rs-0439 */ 
/* FP:config.rs-0440 */     /// The linker features that are disabled on the CLI, using the `-feature` syntax.
/* FP:config.rs-0441 */     pub disabled: LinkerFeatures,
/* FP:config.rs-0442 */ }
/* FP:config.rs-0443 */ 
/* FP:config.rs-0444 */ impl LinkerFeaturesCli {
/* FP:config.rs-0445 */     /// Accumulates an enabled or disabled feature as specified on the CLI, if possible.
/* FP:config.rs-0446 */     /// For example: `+lld`, and `-lld`.
/* FP:config.rs-0447 */     pub(crate) fn handle_cli_feature(&mut self, feature: &str) -> Option<()> {
/* FP:config.rs-0448 */         // Duplicate flags are reduced as we go, the last occurrence wins:
/* FP:config.rs-0449 */         // `+feature,-feature,+feature` only enables the feature, and does not record it as both
/* FP:config.rs-0450 */         // enabled and disabled on the CLI.
/* FP:config.rs-0451 */         // We also only expose `+/-lld` at the moment, as it's currently the only implemented linker
/* FP:config.rs-0452 */         // feature and toggling `LinkerFeatures::CC` would be a noop.
/* FP:config.rs-0453 */         match feature {
/* FP:config.rs-0454 */             "+lld" => {
/* FP:config.rs-0455 */                 self.enabled.insert(LinkerFeatures::LLD);
/* FP:config.rs-0456 */                 self.disabled.remove(LinkerFeatures::LLD);
/* FP:config.rs-0457 */                 Some(())
/* FP:config.rs-0458 */             }
/* FP:config.rs-0459 */             "-lld" => {
/* FP:config.rs-0460 */                 self.disabled.insert(LinkerFeatures::LLD);
/* FP:config.rs-0461 */                 self.enabled.remove(LinkerFeatures::LLD);
/* FP:config.rs-0462 */                 Some(())
/* FP:config.rs-0463 */             }
/* FP:config.rs-0464 */             _ => None,
/* FP:config.rs-0465 */         }
/* FP:config.rs-0466 */     }
/* FP:config.rs-0467 */ 
/* FP:config.rs-0468 */     /// When *not* using `-Z unstable-options` on the CLI, ensure only stable linker features are
/* FP:config.rs-0469 */     /// used, for the given `TargetTuple`. Returns `Ok` if no unstable variants are used.
/* FP:config.rs-0470 */     /// The caller should ensure that e.g. `nightly_options::is_unstable_enabled()`
/* FP:config.rs-0471 */     /// returns false.
/* FP:config.rs-0472 */     pub(crate) fn check_unstable_variants(&self, target_tuple: &TargetTuple) -> Result<(), String> {
/* FP:config.rs-0473 */         // `-C linker-features=-lld` is only stable on x64 linux.
/* FP:config.rs-0474 */         let has_minus_lld = self.disabled.is_lld_enabled();
/* FP:config.rs-0475 */         if has_minus_lld && target_tuple.tuple() != "x86_64-unknown-linux-gnu" {
/* FP:config.rs-0476 */             return Err(format!(
/* FP:config.rs-0477 */                 "`-C linker-features=-lld` is unstable on the `{target_tuple}` \
/* FP:config.rs-0478 */                     target. The `-Z unstable-options` flag must also be passed to use it on this target",
/* FP:config.rs-0479 */             ));
/* FP:config.rs-0480 */         }
/* FP:config.rs-0481 */ 
/* FP:config.rs-0482 */         // Any `+lld` or non-lld feature used is unstable, and that's an error.
/* FP:config.rs-0483 */         let unstable_enabled = self.enabled;
/* FP:config.rs-0484 */         let unstable_disabled = self.disabled - LinkerFeatures::LLD;
/* FP:config.rs-0485 */         if !unstable_enabled.union(unstable_disabled).is_empty() {
/* FP:config.rs-0486 */             let unstable_features: Vec<_> = unstable_enabled
/* FP:config.rs-0487 */                 .iter()
/* FP:config.rs-0488 */                 .map(|f| format!("+{}", f.as_str().unwrap()))
/* FP:config.rs-0489 */                 .chain(unstable_disabled.iter().map(|f| format!("-{}", f.as_str().unwrap())))
/* FP:config.rs-0490 */                 .collect();
/* FP:config.rs-0491 */             return Err(format!(
/* FP:config.rs-0492 */                 "`-C linker-features={}` is unstable, and also requires the \
/* FP:config.rs-0493 */                 `-Z unstable-options` flag to be used",
/* FP:config.rs-0494 */                 unstable_features.join(","),
/* FP:config.rs-0495 */             ));
/* FP:config.rs-0496 */         }
/* FP:config.rs-0497 */ 
/* FP:config.rs-0498 */         Ok(())
/* FP:config.rs-0499 */     }
/* FP:config.rs-0500 */ }
/* FP:config.rs-0501 */ 
/* FP:config.rs-0502 */ /// Used with `-Z assert-incr-state`.
/* FP:config.rs-0503 */ #[derive(Clone, Copy, PartialEq, Hash, Debug)]
/* FP:config.rs-0504 */ pub enum IncrementalStateAssertion {
/* FP:config.rs-0505 */     /// Found and loaded an existing session directory.
/* FP:config.rs-0506 */     ///
/* FP:config.rs-0507 */     /// Note that this says nothing about whether any particular query
/* FP:config.rs-0508 */     /// will be found to be red or green.
/* FP:config.rs-0509 */     Loaded,
/* FP:config.rs-0510 */     /// Did not load an existing session directory.
/* FP:config.rs-0511 */     NotLoaded,
/* FP:config.rs-0512 */ }
/* FP:config.rs-0513 */ 
/* FP:config.rs-0514 */ /// The different settings that can be enabled via the `-Z location-detail` flag.
/* FP:config.rs-0515 */ #[derive(Copy, Clone, PartialEq, Hash, Debug)]
/* FP:config.rs-0516 */ pub struct LocationDetail {
/* FP:config.rs-0517 */     pub file: bool,
/* FP:config.rs-0518 */     pub line: bool,
/* FP:config.rs-0519 */     pub column: bool,
/* FP:config.rs-0520 */ }
/* FP:config.rs-0521 */ 
/* FP:config.rs-0522 */ impl LocationDetail {
/* FP:config.rs-0523 */     pub(crate) fn all() -> Self {
/* FP:config.rs-0524 */         Self { file: true, line: true, column: true }
/* FP:config.rs-0525 */     }
/* FP:config.rs-0526 */ }
/* FP:config.rs-0527 */ 
/* FP:config.rs-0528 */ /// Values for the `-Z fmt-debug` flag.
/* FP:config.rs-0529 */ #[derive(Copy, Clone, PartialEq, Hash, Debug)]
/* FP:config.rs-0530 */ pub enum FmtDebug {
/* FP:config.rs-0531 */     /// Derive fully-featured implementation
/* FP:config.rs-0532 */     Full,
/* FP:config.rs-0533 */     /// Print only type name, without fields
/* FP:config.rs-0534 */     Shallow,
/* FP:config.rs-0535 */     /// `#[derive(Debug)]` and `{:?}` are no-ops
/* FP:config.rs-0536 */     None,
/* FP:config.rs-0537 */ }
/* FP:config.rs-0538 */ 
/* FP:config.rs-0539 */ impl FmtDebug {
/* FP:config.rs-0540 */     pub(crate) fn all() -> [Symbol; 3] {
/* FP:config.rs-0541 */         [sym::full, sym::none, sym::shallow]
/* FP:config.rs-0542 */     }
/* FP:config.rs-0543 */ }
/* FP:config.rs-0544 */ 
/* FP:config.rs-0545 */ #[derive(Clone, PartialEq, Hash, Debug)]
/* FP:config.rs-0546 */ pub enum SwitchWithOptPath {
/* FP:config.rs-0547 */     Enabled(Option<PathBuf>),
/* FP:config.rs-0548 */     Disabled,
/* FP:config.rs-0549 */ }
/* FP:config.rs-0550 */ 
/* FP:config.rs-0551 */ impl SwitchWithOptPath {
/* FP:config.rs-0552 */     pub fn enabled(&self) -> bool {
/* FP:config.rs-0553 */         match *self {
/* FP:config.rs-0554 */             SwitchWithOptPath::Enabled(_) => true,
/* FP:config.rs-0555 */             SwitchWithOptPath::Disabled => false,
/* FP:config.rs-0556 */         }
/* FP:config.rs-0557 */     }
/* FP:config.rs-0558 */ }
/* FP:config.rs-0559 */ 
/* FP:config.rs-0560 */ #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, HashStable_Generic)]
/* FP:config.rs-0561 */ #[derive(Encodable, Decodable)]
/* FP:config.rs-0562 */ pub enum SymbolManglingVersion {
/* FP:config.rs-0563 */     Legacy,
/* FP:config.rs-0564 */     V0,
/* FP:config.rs-0565 */     Hashed,
/* FP:config.rs-0566 */ }
/* FP:config.rs-0567 */ 
/* FP:config.rs-0568 */ #[derive(Clone, Copy, Debug, PartialEq, Hash)]
/* FP:config.rs-0569 */ pub enum DebugInfo {
/* FP:config.rs-0570 */     None,
/* FP:config.rs-0571 */     LineDirectivesOnly,
/* FP:config.rs-0572 */     LineTablesOnly,
/* FP:config.rs-0573 */     Limited,
/* FP:config.rs-0574 */     Full,
/* FP:config.rs-0575 */ }
/* FP:config.rs-0576 */ 
/* FP:config.rs-0577 */ #[derive(Clone, Copy, Debug, PartialEq, Hash)]
/* FP:config.rs-0578 */ pub enum DebugInfoCompression {
/* FP:config.rs-0579 */     None,
/* FP:config.rs-0580 */     Zlib,
/* FP:config.rs-0581 */     Zstd,
/* FP:config.rs-0582 */ }
/* FP:config.rs-0583 */ 
/* FP:config.rs-0584 */ impl ToString for DebugInfoCompression {
/* FP:config.rs-0585 */     fn to_string(&self) -> String {
/* FP:config.rs-0586 */         match self {
/* FP:config.rs-0587 */             DebugInfoCompression::None => "none",
/* FP:config.rs-0588 */             DebugInfoCompression::Zlib => "zlib",
/* FP:config.rs-0589 */             DebugInfoCompression::Zstd => "zstd",
/* FP:config.rs-0590 */         }
/* FP:config.rs-0591 */         .to_owned()
/* FP:config.rs-0592 */     }
/* FP:config.rs-0593 */ }
/* FP:config.rs-0594 */ 
/* FP:config.rs-0595 */ #[derive(Clone, Copy, Debug, PartialEq, Hash)]
/* FP:config.rs-0596 */ pub enum MirStripDebugInfo {
/* FP:config.rs-0597 */     None,
/* FP:config.rs-0598 */     LocalsInTinyFunctions,
/* FP:config.rs-0599 */     AllLocals,
/* FP:config.rs-0600 */ }
/* FP:config.rs-0601 */ 
/* FP:config.rs-0602 */ /// Split debug-information is enabled by `-C split-debuginfo`, this enum is only used if split
/* FP:config.rs-0603 */ /// debug-information is enabled (in either `Packed` or `Unpacked` modes), and the platform
/* FP:config.rs-0604 */ /// uses DWARF for debug-information.
/* FP:config.rs-0605 */ ///
/* FP:config.rs-0606 */ /// Some debug-information requires link-time relocation and some does not. LLVM can partition
/* FP:config.rs-0607 */ /// the debuginfo into sections depending on whether or not it requires link-time relocation. Split
/* FP:config.rs-0608 */ /// DWARF provides a mechanism which allows the linker to skip the sections which don't require
/* FP:config.rs-0609 */ /// link-time relocation - either by putting those sections in DWARF object files, or by keeping
/* FP:config.rs-0610 */ /// them in the object file in such a way that the linker will skip them.
/* FP:config.rs-0611 */ #[derive(Clone, Copy, Debug, PartialEq, Hash)]
/* FP:config.rs-0612 */ pub enum SplitDwarfKind {
/* FP:config.rs-0613 */     /// Sections which do not require relocation are written into object file but ignored by the
/* FP:config.rs-0614 */     /// linker.
/* FP:config.rs-0615 */     Single,
/* FP:config.rs-0616 */     /// Sections which do not require relocation are written into a DWARF object (`.dwo`) file
/* FP:config.rs-0617 */     /// which is ignored by the linker.
/* FP:config.rs-0618 */     Split,
/* FP:config.rs-0619 */ }
/* FP:config.rs-0620 */ 
/* FP:config.rs-0621 */ impl FromStr for SplitDwarfKind {
/* FP:config.rs-0622 */     type Err = ();
/* FP:config.rs-0623 */ 
/* FP:config.rs-0624 */     fn from_str(s: &str) -> Result<Self, ()> {
/* FP:config.rs-0625 */         Ok(match s {
/* FP:config.rs-0626 */             "single" => SplitDwarfKind::Single,
/* FP:config.rs-0627 */             "split" => SplitDwarfKind::Split,
/* FP:config.rs-0628 */             _ => return Err(()),
/* FP:config.rs-0629 */         })
/* FP:config.rs-0630 */     }
/* FP:config.rs-0631 */ }
/* FP:config.rs-0632 */ 
/* FP:config.rs-0633 */ macro_rules! define_output_types {
/* FP:config.rs-0634 */     (
/* FP:config.rs-0635 */         $(
/* FP:config.rs-0636 */             $(#[doc = $doc:expr])*
/* FP:config.rs-0637 */             $Variant:ident => {
/* FP:config.rs-0638 */                 shorthand: $shorthand:expr,
/* FP:config.rs-0639 */                 extension: $extension:expr,
/* FP:config.rs-0640 */                 description: $description:expr,
/* FP:config.rs-0641 */                 default_filename: $default_filename:expr,
/* FP:config.rs-0642 */                 is_text: $is_text:expr,
/* FP:config.rs-0643 */                 compatible_with_cgus_and_single_output: $compatible:expr
/* FP:config.rs-0644 */             }
/* FP:config.rs-0645 */         ),* $(,)?
/* FP:config.rs-0646 */     ) => {
/* FP:config.rs-0647 */         #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, HashStable_Generic)]
/* FP:config.rs-0648 */         #[derive(Encodable, Decodable)]
/* FP:config.rs-0649 */         pub enum OutputType {
/* FP:config.rs-0650 */             $(
/* FP:config.rs-0651 */                 $(#[doc = $doc])*
/* FP:config.rs-0652 */                 $Variant,
/* FP:config.rs-0653 */             )*
/* FP:config.rs-0654 */         }
/* FP:config.rs-0655 */ 
/* FP:config.rs-0656 */ 
/* FP:config.rs-0657 */         impl StableOrd for OutputType {
/* FP:config.rs-0658 */             const CAN_USE_UNSTABLE_SORT: bool = true;
/* FP:config.rs-0659 */ 
/* FP:config.rs-0660 */             // Trivial C-Style enums have a stable sort order across compilation sessions.
/* FP:config.rs-0661 */             const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: () = ();
/* FP:config.rs-0662 */         }
/* FP:config.rs-0663 */ 
/* FP:config.rs-0664 */         impl<HCX: HashStableContext> ToStableHashKey<HCX> for OutputType {
/* FP:config.rs-0665 */             type KeyType = Self;
/* FP:config.rs-0666 */ 
/* FP:config.rs-0667 */             fn to_stable_hash_key(&self, _: &HCX) -> Self::KeyType {
/* FP:config.rs-0668 */                 *self
/* FP:config.rs-0669 */             }
/* FP:config.rs-0670 */         }
/* FP:config.rs-0671 */ 
/* FP:config.rs-0672 */ 
/* FP:config.rs-0673 */         impl OutputType {
/* FP:config.rs-0674 */             pub fn iter_all() -> impl Iterator<Item = OutputType> {
/* FP:config.rs-0675 */                 static ALL_VARIANTS: &[OutputType] = &[
/* FP:config.rs-0676 */                     $(
/* FP:config.rs-0677 */                         OutputType::$Variant,
/* FP:config.rs-0678 */                     )*
/* FP:config.rs-0679 */                 ];
/* FP:config.rs-0680 */                 ALL_VARIANTS.iter().copied()
/* FP:config.rs-0681 */             }
/* FP:config.rs-0682 */ 
/* FP:config.rs-0683 */             fn is_compatible_with_codegen_units_and_single_output_file(&self) -> bool {
/* FP:config.rs-0684 */                 match *self {
/* FP:config.rs-0685 */                     $(
/* FP:config.rs-0686 */                         OutputType::$Variant => $compatible,
/* FP:config.rs-0687 */                     )*
/* FP:config.rs-0688 */                 }
/* FP:config.rs-0689 */             }
/* FP:config.rs-0690 */ 
/* FP:config.rs-0691 */             pub fn shorthand(&self) -> &'static str {
/* FP:config.rs-0692 */                 match *self {
/* FP:config.rs-0693 */                     $(
/* FP:config.rs-0694 */                         OutputType::$Variant => $shorthand,
/* FP:config.rs-0695 */                     )*
/* FP:config.rs-0696 */                 }
/* FP:config.rs-0697 */             }
/* FP:config.rs-0698 */ 
/* FP:config.rs-0699 */             fn from_shorthand(shorthand: &str) -> Option<Self> {
/* FP:config.rs-0700 */                 match shorthand {
/* FP:config.rs-0701 */                     $(
/* FP:config.rs-0702 */                         s if s == $shorthand => Some(OutputType::$Variant),
/* FP:config.rs-0703 */                     )*
/* FP:config.rs-0704 */                     _ => None,
/* FP:config.rs-0705 */                 }
/* FP:config.rs-0706 */             }
/* FP:config.rs-0707 */ 
/* FP:config.rs-0708 */             fn shorthands_display() -> String {
/* FP:config.rs-0709 */                 let shorthands = vec![
/* FP:config.rs-0710 */                     $(
/* FP:config.rs-0711 */                         format!("`{}`", $shorthand),
/* FP:config.rs-0712 */                     )*
/* FP:config.rs-0713 */                 ];
/* FP:config.rs-0714 */                 shorthands.join(", ")
/* FP:config.rs-0715 */             }
/* FP:config.rs-0716 */ 
/* FP:config.rs-0717 */             pub fn extension(&self) -> &'static str {
/* FP:config.rs-0718 */                 match *self {
/* FP:config.rs-0719 */                     $(
/* FP:config.rs-0720 */                         OutputType::$Variant => $extension,
/* FP:config.rs-0721 */                     )*
/* FP:config.rs-0722 */                 }
/* FP:config.rs-0723 */             }
/* FP:config.rs-0724 */ 
/* FP:config.rs-0725 */             pub fn is_text_output(&self) -> bool {
/* FP:config.rs-0726 */                 match *self {
/* FP:config.rs-0727 */                     $(
/* FP:config.rs-0728 */                         OutputType::$Variant => $is_text,
/* FP:config.rs-0729 */                     )*
/* FP:config.rs-0730 */                 }
/* FP:config.rs-0731 */             }
/* FP:config.rs-0732 */ 
/* FP:config.rs-0733 */             pub fn description(&self) -> &'static str {
/* FP:config.rs-0734 */                 match *self {
/* FP:config.rs-0735 */                     $(
/* FP:config.rs-0736 */                         OutputType::$Variant => $description,
/* FP:config.rs-0737 */                     )*
/* FP:config.rs-0738 */                 }
/* FP:config.rs-0739 */             }
/* FP:config.rs-0740 */ 
/* FP:config.rs-0741 */             pub fn default_filename(&self) -> &'static str {
/* FP:config.rs-0742 */                 match *self {
/* FP:config.rs-0743 */                     $(
/* FP:config.rs-0744 */                         OutputType::$Variant => $default_filename,
/* FP:config.rs-0745 */                     )*
/* FP:config.rs-0746 */                 }
/* FP:config.rs-0747 */             }
/* FP:config.rs-0748 */ 
/* FP:config.rs-0749 */ 
/* FP:config.rs-0750 */         }
/* FP:config.rs-0751 */     }
/* FP:config.rs-0752 */ }
/* FP:config.rs-0753 */ 
/* FP:config.rs-0754 */ define_output_types! {
/* FP:config.rs-0755 */     Assembly => {
/* FP:config.rs-0756 */         shorthand: "asm",
/* FP:config.rs-0757 */         extension: "s",
/* FP:config.rs-0758 */         description: "Generates a file with the crate's assembly code",
/* FP:config.rs-0759 */         default_filename: "CRATE_NAME.s",
/* FP:config.rs-0760 */         is_text: true,
/* FP:config.rs-0761 */         compatible_with_cgus_and_single_output: false
/* FP:config.rs-0762 */     },
/* FP:config.rs-0763 */     #[doc = "This is the optimized bitcode, which could be either pre-LTO or non-LTO bitcode,"]
/* FP:config.rs-0764 */     #[doc = "depending on the specific request type."]
/* FP:config.rs-0765 */     Bitcode => {
/* FP:config.rs-0766 */         shorthand: "llvm-bc",
/* FP:config.rs-0767 */         extension: "bc",
/* FP:config.rs-0768 */         description: "Generates a binary file containing the LLVM bitcode",
/* FP:config.rs-0769 */         default_filename: "CRATE_NAME.bc",
/* FP:config.rs-0770 */         is_text: false,
/* FP:config.rs-0771 */         compatible_with_cgus_and_single_output: false
/* FP:config.rs-0772 */     },
/* FP:config.rs-0773 */     DepInfo => {
/* FP:config.rs-0774 */         shorthand: "dep-info",
/* FP:config.rs-0775 */         extension: "d",
/* FP:config.rs-0776 */         description: "Generates a file with Makefile syntax that indicates all the source files that were loaded to generate the crate",
/* FP:config.rs-0777 */         default_filename: "CRATE_NAME.d",
/* FP:config.rs-0778 */         is_text: true,
/* FP:config.rs-0779 */         compatible_with_cgus_and_single_output: true
/* FP:config.rs-0780 */     },
/* FP:config.rs-0781 */     Exe => {
/* FP:config.rs-0782 */         shorthand: "link",
/* FP:config.rs-0783 */         extension: "",
/* FP:config.rs-0784 */         description: "Generates the crates specified by --crate-type. This is the default if --emit is not specified",
/* FP:config.rs-0785 */         default_filename: "(platform and crate-type dependent)",
/* FP:config.rs-0786 */         is_text: false,
/* FP:config.rs-0787 */         compatible_with_cgus_and_single_output: true
/* FP:config.rs-0788 */     },
/* FP:config.rs-0789 */     LlvmAssembly => {
/* FP:config.rs-0790 */         shorthand: "llvm-ir",
/* FP:config.rs-0791 */         extension: "ll",
/* FP:config.rs-0792 */         description: "Generates a file containing LLVM IR",
/* FP:config.rs-0793 */         default_filename: "CRATE_NAME.ll",
/* FP:config.rs-0794 */         is_text: true,
/* FP:config.rs-0795 */         compatible_with_cgus_and_single_output: false
/* FP:config.rs-0796 */     },
/* FP:config.rs-0797 */     Metadata => {
/* FP:config.rs-0798 */         shorthand: "metadata",
/* FP:config.rs-0799 */         extension: "rmeta",
/* FP:config.rs-0800 */         description: "Generates a file containing metadata about the crate",
/* FP:config.rs-0801 */         default_filename: "libCRATE_NAME.rmeta",
/* FP:config.rs-0802 */         is_text: false,
/* FP:config.rs-0803 */         compatible_with_cgus_and_single_output: true
/* FP:config.rs-0804 */     },
/* FP:config.rs-0805 */     Mir => {
/* FP:config.rs-0806 */         shorthand: "mir",
/* FP:config.rs-0807 */         extension: "mir",
/* FP:config.rs-0808 */         description: "Generates a file containing rustc's mid-level intermediate representation",
/* FP:config.rs-0809 */         default_filename: "CRATE_NAME.mir",
/* FP:config.rs-0810 */         is_text: true,
/* FP:config.rs-0811 */         compatible_with_cgus_and_single_output: false
/* FP:config.rs-0812 */     },
/* FP:config.rs-0813 */     Object => {
/* FP:config.rs-0814 */         shorthand: "obj",
/* FP:config.rs-0815 */         extension: "o",
/* FP:config.rs-0816 */         description: "Generates a native object file",
/* FP:config.rs-0817 */         default_filename: "CRATE_NAME.o",
/* FP:config.rs-0818 */         is_text: false,
/* FP:config.rs-0819 */         compatible_with_cgus_and_single_output: false
/* FP:config.rs-0820 */     },
/* FP:config.rs-0821 */     #[doc = "This is the summary or index data part of the ThinLTO bitcode."]
/* FP:config.rs-0822 */     ThinLinkBitcode => {
/* FP:config.rs-0823 */         shorthand: "thin-link-bitcode",
/* FP:config.rs-0824 */         extension: "indexing.o",
/* FP:config.rs-0825 */         description: "Generates the ThinLTO summary as bitcode",
/* FP:config.rs-0826 */         default_filename: "CRATE_NAME.indexing.o",
/* FP:config.rs-0827 */         is_text: false,
/* FP:config.rs-0828 */         compatible_with_cgus_and_single_output: false
/* FP:config.rs-0829 */     },
/* FP:config.rs-0830 */ }
/* FP:config.rs-0831 */ 
/* FP:config.rs-0832 */ /// The type of diagnostics output to generate.
/* FP:config.rs-0833 */ #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
/* FP:config.rs-0834 */ pub enum ErrorOutputType {
/* FP:config.rs-0835 */     /// Output meant for the consumption of humans.
/* FP:config.rs-0836 */     #[default]
/* FP:config.rs-0837 */     HumanReadable {
/* FP:config.rs-0838 */         kind: HumanReadableErrorType = HumanReadableErrorType::Default,
/* FP:config.rs-0839 */         color_config: ColorConfig = ColorConfig::Auto,
/* FP:config.rs-0840 */     },
/* FP:config.rs-0841 */     /// Output that's consumed by other tools such as `rustfix` or the `RLS`.
/* FP:config.rs-0842 */     Json {
/* FP:config.rs-0843 */         /// Render the JSON in a human readable way (with indents and newlines).
/* FP:config.rs-0844 */         pretty: bool,
/* FP:config.rs-0845 */         /// The JSON output includes a `rendered` field that includes the rendered
/* FP:config.rs-0846 */         /// human output.
/* FP:config.rs-0847 */         json_rendered: HumanReadableErrorType,
/* FP:config.rs-0848 */         color_config: ColorConfig,
/* FP:config.rs-0849 */     },
/* FP:config.rs-0850 */ }
/* FP:config.rs-0851 */ 
/* FP:config.rs-0852 */ #[derive(Clone, Hash, Debug)]
/* FP:config.rs-0853 */ pub enum ResolveDocLinks {
/* FP:config.rs-0854 */     /// Do not resolve doc links.
/* FP:config.rs-0855 */     None,
/* FP:config.rs-0856 */     /// Resolve doc links on exported items only for crate types that have metadata.
/* FP:config.rs-0857 */     ExportedMetadata,
/* FP:config.rs-0858 */     /// Resolve doc links on exported items.
/* FP:config.rs-0859 */     Exported,
/* FP:config.rs-0860 */     /// Resolve doc links on all items.
/* FP:config.rs-0861 */     All,
/* FP:config.rs-0862 */ }
/* FP:config.rs-0863 */ 
/* FP:config.rs-0864 */ /// Use tree-based collections to cheaply get a deterministic `Hash` implementation.
/* FP:config.rs-0865 */ /// *Do not* switch `BTreeMap` out for an unsorted container type! That would break
/* FP:config.rs-0866 */ /// dependency tracking for command-line arguments. Also only hash keys, since tracking
/* FP:config.rs-0867 */ /// should only depend on the output types, not the paths they're written to.
/* FP:config.rs-0868 */ #[derive(Clone, Debug, Hash, HashStable_Generic, Encodable, Decodable)]
/* FP:config.rs-0869 */ pub struct OutputTypes(BTreeMap<OutputType, Option<OutFileName>>);
/* FP:config.rs-0870 */ 
/* FP:config.rs-0871 */ impl OutputTypes {
/* FP:config.rs-0872 */     pub fn new(entries: &[(OutputType, Option<OutFileName>)]) -> OutputTypes {
/* FP:config.rs-0873 */         OutputTypes(BTreeMap::from_iter(entries.iter().map(|&(k, ref v)| (k, v.clone()))))
/* FP:config.rs-0874 */     }
/* FP:config.rs-0875 */ 
/* FP:config.rs-0876 */     pub(crate) fn get(&self, key: &OutputType) -> Option<&Option<OutFileName>> {
/* FP:config.rs-0877 */         self.0.get(key)
/* FP:config.rs-0878 */     }
/* FP:config.rs-0879 */ 
/* FP:config.rs-0880 */     pub fn contains_key(&self, key: &OutputType) -> bool {
/* FP:config.rs-0881 */         self.0.contains_key(key)
/* FP:config.rs-0882 */     }
/* FP:config.rs-0883 */ 
/* FP:config.rs-0884 */     /// Returns `true` if user specified a name and not just produced type
/* FP:config.rs-0885 */     pub fn contains_explicit_name(&self, key: &OutputType) -> bool {
/* FP:config.rs-0886 */         matches!(self.0.get(key), Some(Some(..)))
/* FP:config.rs-0887 */     }
/* FP:config.rs-0888 */ 
/* FP:config.rs-0889 */     pub fn iter(&self) -> BTreeMapIter<'_, OutputType, Option<OutFileName>> {
/* FP:config.rs-0890 */         self.0.iter()
/* FP:config.rs-0891 */     }
/* FP:config.rs-0892 */ 
/* FP:config.rs-0893 */     pub fn keys(&self) -> BTreeMapKeysIter<'_, OutputType, Option<OutFileName>> {
/* FP:config.rs-0894 */         self.0.keys()
/* FP:config.rs-0895 */     }
/* FP:config.rs-0896 */ 
/* FP:config.rs-0897 */     pub fn values(&self) -> BTreeMapValuesIter<'_, OutputType, Option<OutFileName>> {
/* FP:config.rs-0898 */         self.0.values()
/* FP:config.rs-0899 */     }
/* FP:config.rs-0900 */ 
/* FP:config.rs-0901 */     pub fn len(&self) -> usize {
/* FP:config.rs-0902 */         self.0.len()
/* FP:config.rs-0903 */     }
/* FP:config.rs-0904 */ 
/* FP:config.rs-0905 */     /// Returns `true` if any of the output types require codegen or linking.
/* FP:config.rs-0906 */     pub fn should_codegen(&self) -> bool {
/* FP:config.rs-0907 */         self.0.keys().any(|k| match *k {
/* FP:config.rs-0908 */             OutputType::Bitcode
/* FP:config.rs-0909 */             | OutputType::ThinLinkBitcode
/* FP:config.rs-0910 */             | OutputType::Assembly
/* FP:config.rs-0911 */             | OutputType::LlvmAssembly
/* FP:config.rs-0912 */             | OutputType::Mir
/* FP:config.rs-0913 */             | OutputType::Object
/* FP:config.rs-0914 */             | OutputType::Exe => true,
/* FP:config.rs-0915 */             OutputType::Metadata | OutputType::DepInfo => false,
/* FP:config.rs-0916 */         })
/* FP:config.rs-0917 */     }
/* FP:config.rs-0918 */ 
/* FP:config.rs-0919 */     /// Returns `true` if any of the output types require linking.
/* FP:config.rs-0920 */     pub fn should_link(&self) -> bool {
/* FP:config.rs-0921 */         self.0.keys().any(|k| match *k {
/* FP:config.rs-0922 */             OutputType::Bitcode
/* FP:config.rs-0923 */             | OutputType::ThinLinkBitcode
/* FP:config.rs-0924 */             | OutputType::Assembly
/* FP:config.rs-0925 */             | OutputType::LlvmAssembly
/* FP:config.rs-0926 */             | OutputType::Mir
/* FP:config.rs-0927 */             | OutputType::Metadata
/* FP:config.rs-0928 */             | OutputType::Object
/* FP:config.rs-0929 */             | OutputType::DepInfo => false,
/* FP:config.rs-0930 */             OutputType::Exe => true,
/* FP:config.rs-0931 */         })
/* FP:config.rs-0932 */     }
/* FP:config.rs-0933 */ }
/* FP:config.rs-0934 */ 
/* FP:config.rs-0935 */ /// Use tree-based collections to cheaply get a deterministic `Hash` implementation.
/* FP:config.rs-0936 */ /// *Do not* switch `BTreeMap` or `BTreeSet` out for an unsorted container type! That
/* FP:config.rs-0937 */ /// would break dependency tracking for command-line arguments.
/* FP:config.rs-0938 */ #[derive(Clone)]
/* FP:config.rs-0939 */ pub struct Externs(BTreeMap<String, ExternEntry>);
/* FP:config.rs-0940 */ 
/* FP:config.rs-0941 */ #[derive(Clone, Debug)]
/* FP:config.rs-0942 */ pub struct ExternEntry {
/* FP:config.rs-0943 */     pub location: ExternLocation,
/* FP:config.rs-0944 */     /// Indicates this is a "private" dependency for the
/* FP:config.rs-0945 */     /// `exported_private_dependencies` lint.
/* FP:config.rs-0946 */     ///
/* FP:config.rs-0947 */     /// This can be set with the `priv` option like
/* FP:config.rs-0948 */     /// `--extern priv:name=foo.rlib`.
/* FP:config.rs-0949 */     pub is_private_dep: bool,
/* FP:config.rs-0950 */     /// Add the extern entry to the extern prelude.
/* FP:config.rs-0951 */     ///
/* FP:config.rs-0952 */     /// This can be disabled with the `noprelude` option like
/* FP:config.rs-0953 */     /// `--extern noprelude:name`.
/* FP:config.rs-0954 */     pub add_prelude: bool,
/* FP:config.rs-0955 */     /// The extern entry shouldn't be considered for unused dependency warnings.
/* FP:config.rs-0956 */     ///
/* FP:config.rs-0957 */     /// `--extern nounused:std=/path/to/lib/libstd.rlib`. This is used to
/* FP:config.rs-0958 */     /// suppress `unused-crate-dependencies` warnings.
/* FP:config.rs-0959 */     pub nounused_dep: bool,
/* FP:config.rs-0960 */     /// If the extern entry is not referenced in the crate, force it to be resolved anyway.
/* FP:config.rs-0961 */     ///
/* FP:config.rs-0962 */     /// Allows a dependency satisfying, for instance, a missing panic handler to be injected
/* FP:config.rs-0963 */     /// without modifying source:
/* FP:config.rs-0964 */     /// `--extern force:extras=/path/to/lib/libstd.rlib`
/* FP:config.rs-0965 */     pub force: bool,
/* FP:config.rs-0966 */ }
/* FP:config.rs-0967 */ 
/* FP:config.rs-0968 */ #[derive(Clone, Debug)]
/* FP:config.rs-0969 */ pub enum ExternLocation {
/* FP:config.rs-0970 */     /// Indicates to look for the library in the search paths.
/* FP:config.rs-0971 */     ///
/* FP:config.rs-0972 */     /// Added via `--extern name`.
/* FP:config.rs-0973 */     FoundInLibrarySearchDirectories,
/* FP:config.rs-0974 */     /// The locations where this extern entry must be found.
/* FP:config.rs-0975 */     ///
/* FP:config.rs-0976 */     /// The `CrateLoader` is responsible for loading these and figuring out
/* FP:config.rs-0977 */     /// which one to use.
/* FP:config.rs-0978 */     ///
/* FP:config.rs-0979 */     /// Added via `--extern prelude_name=some_file.rlib`
/* FP:config.rs-0980 */     ExactPaths(BTreeSet<CanonicalizedPath>),
/* FP:config.rs-0981 */ }
/* FP:config.rs-0982 */ 
/* FP:config.rs-0983 */ impl Externs {
/* FP:config.rs-0984 */     /// Used for testing.
/* FP:config.rs-0985 */     pub fn new(data: BTreeMap<String, ExternEntry>) -> Externs {
/* FP:config.rs-0986 */         Externs(data)
/* FP:config.rs-0987 */     }
/* FP:config.rs-0988 */ 
/* FP:config.rs-0989 */     pub fn get(&self, key: &str) -> Option<&ExternEntry> {
/* FP:config.rs-0990 */         self.0.get(key)
/* FP:config.rs-0991 */     }
/* FP:config.rs-0992 */ 
/* FP:config.rs-0993 */     pub fn iter(&self) -> BTreeMapIter<'_, String, ExternEntry> {
/* FP:config.rs-0994 */         self.0.iter()
/* FP:config.rs-0995 */     }
/* FP:config.rs-0996 */ }
/* FP:config.rs-0997 */ 
/* FP:config.rs-0998 */ impl ExternEntry {
/* FP:config.rs-0999 */     fn new(location: ExternLocation) -> ExternEntry {
/* FP:config.rs-1000 */         ExternEntry {
/* FP:config.rs-1001 */             location,
/* FP:config.rs-1002 */             is_private_dep: false,
/* FP:config.rs-1003 */             add_prelude: false,
/* FP:config.rs-1004 */             nounused_dep: false,
/* FP:config.rs-1005 */             force: false,
/* FP:config.rs-1006 */         }
/* FP:config.rs-1007 */     }
/* FP:config.rs-1008 */ 
/* FP:config.rs-1009 */     pub fn files(&self) -> Option<impl Iterator<Item = &CanonicalizedPath>> {
/* FP:config.rs-1010 */         match &self.location {
/* FP:config.rs-1011 */             ExternLocation::ExactPaths(set) => Some(set.iter()),
/* FP:config.rs-1012 */             _ => None,
/* FP:config.rs-1013 */         }
/* FP:config.rs-1014 */     }
/* FP:config.rs-1015 */ }
/* FP:config.rs-1016 */ 
/* FP:config.rs-1017 */ #[derive(Clone, PartialEq, Debug)]
/* FP:config.rs-1018 */ pub struct PrintRequest {
/* FP:config.rs-1019 */     pub kind: PrintKind,
/* FP:config.rs-1020 */     pub out: OutFileName,
/* FP:config.rs-1021 */ }
/* FP:config.rs-1022 */ 
/* FP:config.rs-1023 */ #[derive(Copy, Clone, PartialEq, Eq, Debug)]
/* FP:config.rs-1024 */ pub enum PrintKind {
/* FP:config.rs-1025 */     // tidy-alphabetical-start
/* FP:config.rs-1026 */     AllTargetSpecsJson,
/* FP:config.rs-1027 */     CallingConventions,
/* FP:config.rs-1028 */     Cfg,
/* FP:config.rs-1029 */     CheckCfg,
/* FP:config.rs-1030 */     CodeModels,
/* FP:config.rs-1031 */     CrateName,
/* FP:config.rs-1032 */     CrateRootLintLevels,
/* FP:config.rs-1033 */     DeploymentTarget,
/* FP:config.rs-1034 */     FileNames,
/* FP:config.rs-1035 */     HostTuple,
/* FP:config.rs-1036 */     LinkArgs,
/* FP:config.rs-1037 */     NativeStaticLibs,
/* FP:config.rs-1038 */     RelocationModels,
/* FP:config.rs-1039 */     SplitDebuginfo,
/* FP:config.rs-1040 */     StackProtectorStrategies,
/* FP:config.rs-1041 */     SupportedCrateTypes,
/* FP:config.rs-1042 */     Sysroot,
/* FP:config.rs-1043 */     TargetCPUs,
/* FP:config.rs-1044 */     TargetFeatures,
/* FP:config.rs-1045 */     TargetLibdir,
/* FP:config.rs-1046 */     TargetList,
/* FP:config.rs-1047 */     TargetSpecJson,
/* FP:config.rs-1048 */     TargetSpecJsonSchema,
/* FP:config.rs-1049 */     TlsModels,
/* FP:config.rs-1050 */     // tidy-alphabetical-end
/* FP:config.rs-1051 */ }
/* FP:config.rs-1052 */ 
/* FP:config.rs-1053 */ #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
/* FP:config.rs-1054 */ pub struct NextSolverConfig {
/* FP:config.rs-1055 */     /// Whether the new trait solver should be enabled in coherence.
/* FP:config.rs-1056 */     pub coherence: bool = true,
/* FP:config.rs-1057 */     /// Whether the new trait solver should be enabled everywhere.
/* FP:config.rs-1058 */     /// This is only `true` if `coherence` is also enabled.
/* FP:config.rs-1059 */     pub globally: bool = false,
/* FP:config.rs-1060 */ }
/* FP:config.rs-1061 */ 
/* FP:config.rs-1062 */ #[derive(Clone)]
/* FP:config.rs-1063 */ pub enum Input {
/* FP:config.rs-1064 */     /// Load source code from a file.
/* FP:config.rs-1065 */     File(PathBuf),
/* FP:config.rs-1066 */     /// Load source code from a string.
/* FP:config.rs-1067 */     Str {
/* FP:config.rs-1068 */         /// A string that is shown in place of a filename.
/* FP:config.rs-1069 */         name: FileName,
/* FP:config.rs-1070 */         /// An anonymous string containing the source code.
/* FP:config.rs-1071 */         input: String,
/* FP:config.rs-1072 */     },
/* FP:config.rs-1073 */ }
/* FP:config.rs-1074 */ 
/* FP:config.rs-1075 */ impl Input {
/* FP:config.rs-1076 */     pub fn filestem(&self) -> &str {
/* FP:config.rs-1077 */         if let Input::File(ifile) = self {
/* FP:config.rs-1078 */             // If for some reason getting the file stem as a UTF-8 string fails,
/* FP:config.rs-1079 */             // then fallback to a fixed name.
/* FP:config.rs-1080 */             if let Some(name) = ifile.file_stem().and_then(OsStr::to_str) {
/* FP:config.rs-1081 */                 return name;
/* FP:config.rs-1082 */             }
/* FP:config.rs-1083 */         }
/* FP:config.rs-1084 */         "rust_out"
/* FP:config.rs-1085 */     }
/* FP:config.rs-1086 */ 
/* FP:config.rs-1087 */     pub fn source_name(&self) -> FileName {
/* FP:config.rs-1088 */         match *self {
/* FP:config.rs-1089 */             Input::File(ref ifile) => ifile.clone().into(),
/* FP:config.rs-1090 */             Input::Str { ref name, .. } => name.clone(),
/* FP:config.rs-1091 */         }
/* FP:config.rs-1092 */     }
/* FP:config.rs-1093 */ 
/* FP:config.rs-1094 */     pub fn opt_path(&self) -> Option<&Path> {
/* FP:config.rs-1095 */         match self {
/* FP:config.rs-1096 */             Input::File(file) => Some(file),
/* FP:config.rs-1097 */             Input::Str { name, .. } => match name {
/* FP:config.rs-1098 */                 FileName::Real(real) => real.local_path(),
/* FP:config.rs-1099 */                 FileName::CfgSpec(_) => None,
/* FP:config.rs-1100 */                 FileName::Anon(_) => None,
/* FP:config.rs-1101 */                 FileName::MacroExpansion(_) => None,
/* FP:config.rs-1102 */                 FileName::ProcMacroSourceCode(_) => None,
/* FP:config.rs-1103 */                 FileName::CliCrateAttr(_) => None,
/* FP:config.rs-1104 */                 FileName::Custom(_) => None,
/* FP:config.rs-1105 */                 FileName::DocTest(path, _) => Some(path),
/* FP:config.rs-1106 */                 FileName::InlineAsm(_) => None,
/* FP:config.rs-1107 */             },
/* FP:config.rs-1108 */         }
/* FP:config.rs-1109 */     }
/* FP:config.rs-1110 */ }
/* FP:config.rs-1111 */ 
/* FP:config.rs-1112 */ #[derive(Clone, Hash, Debug, HashStable_Generic, PartialEq, Encodable, Decodable)]
/* FP:config.rs-1113 */ pub enum OutFileName {
/* FP:config.rs-1114 */     Real(PathBuf),
/* FP:config.rs-1115 */     Stdout,
/* FP:config.rs-1116 */ }
/* FP:config.rs-1117 */ 
/* FP:config.rs-1118 */ impl OutFileName {
/* FP:config.rs-1119 */     pub fn parent(&self) -> Option<&Path> {
/* FP:config.rs-1120 */         match *self {
/* FP:config.rs-1121 */             OutFileName::Real(ref path) => path.parent(),
/* FP:config.rs-1122 */             OutFileName::Stdout => None,
/* FP:config.rs-1123 */         }
/* FP:config.rs-1124 */     }
/* FP:config.rs-1125 */ 
/* FP:config.rs-1126 */     pub fn filestem(&self) -> Option<&OsStr> {
/* FP:config.rs-1127 */         match *self {
/* FP:config.rs-1128 */             OutFileName::Real(ref path) => path.file_stem(),
/* FP:config.rs-1129 */             OutFileName::Stdout => Some(OsStr::new("stdout")),
/* FP:config.rs-1130 */         }
/* FP:config.rs-1131 */     }
/* FP:config.rs-1132 */ 
/* FP:config.rs-1133 */     pub fn is_stdout(&self) -> bool {
/* FP:config.rs-1134 */         match *self {
/* FP:config.rs-1135 */             OutFileName::Real(_) => false,
/* FP:config.rs-1136 */             OutFileName::Stdout => true,
/* FP:config.rs-1137 */         }
/* FP:config.rs-1138 */     }
/* FP:config.rs-1139 */ 
/* FP:config.rs-1140 */     pub fn is_tty(&self) -> bool {
/* FP:config.rs-1141 */         use std::io::IsTerminal;
/* FP:config.rs-1142 */         match *self {
/* FP:config.rs-1143 */             OutFileName::Real(_) => false,
/* FP:config.rs-1144 */             OutFileName::Stdout => std::io::stdout().is_terminal(),
/* FP:config.rs-1145 */         }
/* FP:config.rs-1146 */     }
/* FP:config.rs-1147 */ 
/* FP:config.rs-1148 */     pub fn as_path(&self) -> &Path {
/* FP:config.rs-1149 */         match *self {
/* FP:config.rs-1150 */             OutFileName::Real(ref path) => path.as_ref(),
/* FP:config.rs-1151 */             OutFileName::Stdout => Path::new("stdout"),
/* FP:config.rs-1152 */         }
/* FP:config.rs-1153 */     }
/* FP:config.rs-1154 */ 
/* FP:config.rs-1155 */     /// For a given output filename, return the actual name of the file that
/* FP:config.rs-1156 */     /// can be used to write codegen data of type `flavor`. For real-path
/* FP:config.rs-1157 */     /// output filenames, this would be trivial as we can just use the path.
/* FP:config.rs-1158 */     /// Otherwise for stdout, return a temporary path so that the codegen data
/* FP:config.rs-1159 */     /// may be later copied to stdout.
/* FP:config.rs-1160 */     pub fn file_for_writing(
/* FP:config.rs-1161 */         &self,
/* FP:config.rs-1162 */         outputs: &OutputFilenames,
/* FP:config.rs-1163 */         flavor: OutputType,
/* FP:config.rs-1164 */         codegen_unit_name: &str,
/* FP:config.rs-1165 */         invocation_temp: Option<&str>,
/* FP:config.rs-1166 */     ) -> PathBuf {
/* FP:config.rs-1167 */         match *self {
/* FP:config.rs-1168 */             OutFileName::Real(ref path) => path.clone(),
/* FP:config.rs-1169 */             OutFileName::Stdout => {
/* FP:config.rs-1170 */                 outputs.temp_path_for_cgu(flavor, codegen_unit_name, invocation_temp)
/* FP:config.rs-1171 */             }
/* FP:config.rs-1172 */         }
/* FP:config.rs-1173 */     }
/* FP:config.rs-1174 */ 
/* FP:config.rs-1175 */     pub fn overwrite(&self, content: &str, sess: &Session) {
/* FP:config.rs-1176 */         match self {
/* FP:config.rs-1177 */             OutFileName::Stdout => print!("{content}"),
/* FP:config.rs-1178 */             OutFileName::Real(path) => {
/* FP:config.rs-1179 */                 if let Err(e) = fs::write(path, content) {
/* FP:config.rs-1180 */                     sess.dcx().emit_fatal(FileWriteFail { path, err: e.to_string() });
/* FP:config.rs-1181 */                 }
/* FP:config.rs-1182 */             }
/* FP:config.rs-1183 */         }
/* FP:config.rs-1184 */     }
/* FP:config.rs-1185 */ }
/* FP:config.rs-1186 */ 
/* FP:config.rs-1187 */ #[derive(Clone, Hash, Debug, HashStable_Generic, Encodable, Decodable)]
/* FP:config.rs-1188 */ pub struct OutputFilenames {
/* FP:config.rs-1189 */     pub(crate) out_directory: PathBuf,
/* FP:config.rs-1190 */     /// Crate name. Never contains '-'.
/* FP:config.rs-1191 */     crate_stem: String,
/* FP:config.rs-1192 */     /// Typically based on `.rs` input file name. Any '-' is preserved.
/* FP:config.rs-1193 */     filestem: String,
/* FP:config.rs-1194 */     pub single_output_file: Option<OutFileName>,
/* FP:config.rs-1195 */     temps_directory: Option<PathBuf>,
/* FP:config.rs-1196 */     pub outputs: OutputTypes,
/* FP:config.rs-1197 */ }
/* FP:config.rs-1198 */ 
/* FP:config.rs-1199 */ pub const RLINK_EXT: &str = "rlink";
/* FP:config.rs-1200 */ pub const RUST_CGU_EXT: &str = "rcgu";
/* FP:config.rs-1201 */ pub const DWARF_OBJECT_EXT: &str = "dwo";
/* FP:config.rs-1202 */ pub const MAX_FILENAME_LENGTH: usize = 143; // ecryptfs limits filenames to 143 bytes see #49914
/* FP:config.rs-1203 */ 
/* FP:config.rs-1204 */ /// Ensure the filename is not too long, as some filesystems have a limit.
/* FP:config.rs-1205 */ /// If the filename is too long, hash part of it and append the hash to the filename.
/* FP:config.rs-1206 */ /// This is a workaround for long crate names generating overly long filenames.
/* FP:config.rs-1207 */ fn maybe_strip_file_name(mut path: PathBuf) -> PathBuf {
/* FP:config.rs-1208 */     if path.file_name().map_or(0, |name| name.len()) > MAX_FILENAME_LENGTH {
/* FP:config.rs-1209 */         let filename = path.file_name().unwrap().to_string_lossy();
/* FP:config.rs-1210 */         let hash_len = 64 / 4; // Hash64 is 64 bits encoded in hex
/* FP:config.rs-1211 */         let hyphen_len = 1; // the '-' we insert between hash and suffix
/* FP:config.rs-1212 */ 
/* FP:config.rs-1213 */         // number of bytes of suffix we can keep so that "hash-<suffix>" fits
/* FP:config.rs-1214 */         let allowed_suffix = MAX_FILENAME_LENGTH.saturating_sub(hash_len + hyphen_len);
/* FP:config.rs-1215 */ 
/* FP:config.rs-1216 */         // number of bytes to remove from the start
/* FP:config.rs-1217 */         let stripped_bytes = filename.len().saturating_sub(allowed_suffix);
/* FP:config.rs-1218 */ 
/* FP:config.rs-1219 */         // ensure we don't cut in a middle of a char
/* FP:config.rs-1220 */         let split_at = filename.ceil_char_boundary(stripped_bytes);
/* FP:config.rs-1221 */ 
/* FP:config.rs-1222 */         let mut hasher = StableHasher::new();
/* FP:config.rs-1223 */         filename[..split_at].hash(&mut hasher);
/* FP:config.rs-1224 */         let hash = hasher.finish::<Hash64>();
/* FP:config.rs-1225 */ 
/* FP:config.rs-1226 */         path.set_file_name(format!("{:x}-{}", hash, &filename[split_at..]));
/* FP:config.rs-1227 */     }
/* FP:config.rs-1228 */     path
/* FP:config.rs-1229 */ }
/* FP:config.rs-1230 */ impl OutputFilenames {
/* FP:config.rs-1231 */     pub fn new(
/* FP:config.rs-1232 */         out_directory: PathBuf,
/* FP:config.rs-1233 */         out_crate_name: String,
/* FP:config.rs-1234 */         out_filestem: String,
/* FP:config.rs-1235 */         single_output_file: Option<OutFileName>,
/* FP:config.rs-1236 */         temps_directory: Option<PathBuf>,
/* FP:config.rs-1237 */         extra: String,
/* FP:config.rs-1238 */         outputs: OutputTypes,
/* FP:config.rs-1239 */     ) -> Self {
/* FP:config.rs-1240 */         OutputFilenames {
/* FP:config.rs-1241 */             out_directory,
/* FP:config.rs-1242 */             single_output_file,
/* FP:config.rs-1243 */             temps_directory,
/* FP:config.rs-1244 */             outputs,
/* FP:config.rs-1245 */             crate_stem: format!("{out_crate_name}{extra}"),
/* FP:config.rs-1246 */             filestem: format!("{out_filestem}{extra}"),
/* FP:config.rs-1247 */         }
/* FP:config.rs-1248 */     }
/* FP:config.rs-1249 */ 
/* FP:config.rs-1250 */     pub fn path(&self, flavor: OutputType) -> OutFileName {
/* FP:config.rs-1251 */         self.outputs
/* FP:config.rs-1252 */             .get(&flavor)
/* FP:config.rs-1253 */             .and_then(|p| p.to_owned())
/* FP:config.rs-1254 */             .or_else(|| self.single_output_file.clone())
/* FP:config.rs-1255 */             .unwrap_or_else(|| OutFileName::Real(self.output_path(flavor)))
/* FP:config.rs-1256 */     }
/* FP:config.rs-1257 */ 
/* FP:config.rs-1258 */     pub fn interface_path(&self) -> PathBuf {
/* FP:config.rs-1259 */         self.out_directory.join(format!("lib{}.rs", self.crate_stem))
/* FP:config.rs-1260 */     }
/* FP:config.rs-1261 */ 
/* FP:config.rs-1262 */     /// Gets the output path where a compilation artifact of the given type
/* FP:config.rs-1263 */     /// should be placed on disk.
/* FP:config.rs-1264 */     fn output_path(&self, flavor: OutputType) -> PathBuf {
/* FP:config.rs-1265 */         let extension = flavor.extension();
/* FP:config.rs-1266 */         match flavor {
/* FP:config.rs-1267 */             OutputType::Metadata => {
/* FP:config.rs-1268 */                 self.out_directory.join(format!("lib{}.{}", self.crate_stem, extension))
/* FP:config.rs-1269 */             }
/* FP:config.rs-1270 */             _ => self.with_directory_and_extension(&self.out_directory, extension),
/* FP:config.rs-1271 */         }
/* FP:config.rs-1272 */     }
/* FP:config.rs-1273 */ 
/* FP:config.rs-1274 */     /// Gets the path where a compilation artifact of the given type for the
/* FP:config.rs-1275 */     /// given codegen unit should be placed on disk. If codegen_unit_name is
/* FP:config.rs-1276 */     /// None, a path distinct from those of any codegen unit will be generated.
/* FP:config.rs-1277 */     pub fn temp_path_for_cgu(
/* FP:config.rs-1278 */         &self,
/* FP:config.rs-1279 */         flavor: OutputType,
/* FP:config.rs-1280 */         codegen_unit_name: &str,
/* FP:config.rs-1281 */         invocation_temp: Option<&str>,
/* FP:config.rs-1282 */     ) -> PathBuf {
/* FP:config.rs-1283 */         let extension = flavor.extension();
/* FP:config.rs-1284 */         self.temp_path_ext_for_cgu(extension, codegen_unit_name, invocation_temp)
/* FP:config.rs-1285 */     }
/* FP:config.rs-1286 */ 
/* FP:config.rs-1287 */     /// Like `temp_path`, but specifically for dwarf objects.
/* FP:config.rs-1288 */     pub fn temp_path_dwo_for_cgu(
/* FP:config.rs-1289 */         &self,
/* FP:config.rs-1290 */         codegen_unit_name: &str,
/* FP:config.rs-1291 */         invocation_temp: Option<&str>,
/* FP:config.rs-1292 */     ) -> PathBuf {
/* FP:config.rs-1293 */         self.temp_path_ext_for_cgu(DWARF_OBJECT_EXT, codegen_unit_name, invocation_temp)
/* FP:config.rs-1294 */     }
/* FP:config.rs-1295 */ 
/* FP:config.rs-1296 */     /// Like `temp_path`, but also supports things where there is no corresponding
/* FP:config.rs-1297 */     /// OutputType, like noopt-bitcode or lto-bitcode.
/* FP:config.rs-1298 */     pub fn temp_path_ext_for_cgu(
/* FP:config.rs-1299 */         &self,
/* FP:config.rs-1300 */         ext: &str,
/* FP:config.rs-1301 */         codegen_unit_name: &str,
/* FP:config.rs-1302 */         invocation_temp: Option<&str>,
/* FP:config.rs-1303 */     ) -> PathBuf {
/* FP:config.rs-1304 */         let mut extension = codegen_unit_name.to_string();
/* FP:config.rs-1305 */ 
/* FP:config.rs-1306 */         // Append `.{invocation_temp}` to ensure temporary files are unique.
/* FP:config.rs-1307 */         if let Some(rng) = invocation_temp {
/* FP:config.rs-1308 */             extension.push('.');
/* FP:config.rs-1309 */             extension.push_str(rng);
/* FP:config.rs-1310 */         }
/* FP:config.rs-1311 */ 
/* FP:config.rs-1312 */         // FIXME: This is sketchy that we're not appending `.rcgu` when the ext is empty.
/* FP:config.rs-1313 */         // Append `.rcgu.{ext}`.
/* FP:config.rs-1314 */         if !ext.is_empty() {
/* FP:config.rs-1315 */             extension.push('.');
/* FP:config.rs-1316 */             extension.push_str(RUST_CGU_EXT);
/* FP:config.rs-1317 */             extension.push('.');
/* FP:config.rs-1318 */             extension.push_str(ext);
/* FP:config.rs-1319 */         }
/* FP:config.rs-1320 */ 
/* FP:config.rs-1321 */         let temps_directory = self.temps_directory.as_ref().unwrap_or(&self.out_directory);
/* FP:config.rs-1322 */         maybe_strip_file_name(self.with_directory_and_extension(temps_directory, &extension))
/* FP:config.rs-1323 */     }
/* FP:config.rs-1324 */ 
/* FP:config.rs-1325 */     pub fn temp_path_for_diagnostic(&self, ext: &str) -> PathBuf {
/* FP:config.rs-1326 */         let temps_directory = self.temps_directory.as_ref().unwrap_or(&self.out_directory);
/* FP:config.rs-1327 */         self.with_directory_and_extension(temps_directory, &ext)
/* FP:config.rs-1328 */     }
/* FP:config.rs-1329 */ 
/* FP:config.rs-1330 */     pub fn with_extension(&self, extension: &str) -> PathBuf {
/* FP:config.rs-1331 */         self.with_directory_and_extension(&self.out_directory, extension)
/* FP:config.rs-1332 */     }
/* FP:config.rs-1333 */ 
/* FP:config.rs-1334 */     pub fn with_directory_and_extension(&self, directory: &Path, extension: &str) -> PathBuf {
/* FP:config.rs-1335 */         let mut path = directory.join(&self.filestem);
/* FP:config.rs-1336 */         path.set_extension(extension);
/* FP:config.rs-1337 */         path
/* FP:config.rs-1338 */     }
/* FP:config.rs-1339 */ 
/* FP:config.rs-1340 */     /// Returns the path for the Split DWARF file - this can differ depending on which Split DWARF
/* FP:config.rs-1341 */     /// mode is being used, which is the logic that this function is intended to encapsulate.
/* FP:config.rs-1342 */     pub fn split_dwarf_path(
/* FP:config.rs-1343 */         &self,
/* FP:config.rs-1344 */         split_debuginfo_kind: SplitDebuginfo,
/* FP:config.rs-1345 */         split_dwarf_kind: SplitDwarfKind,
/* FP:config.rs-1346 */         cgu_name: &str,
/* FP:config.rs-1347 */         invocation_temp: Option<&str>,
/* FP:config.rs-1348 */     ) -> Option<PathBuf> {
/* FP:config.rs-1349 */         let obj_out = self.temp_path_for_cgu(OutputType::Object, cgu_name, invocation_temp);
/* FP:config.rs-1350 */         let dwo_out = self.temp_path_dwo_for_cgu(cgu_name, invocation_temp);
/* FP:config.rs-1351 */         match (split_debuginfo_kind, split_dwarf_kind) {
/* FP:config.rs-1352 */             (SplitDebuginfo::Off, SplitDwarfKind::Single | SplitDwarfKind::Split) => None,
/* FP:config.rs-1353 */             // Single mode doesn't change how DWARF is emitted, but does add Split DWARF attributes
/* FP:config.rs-1354 */             // (pointing at the path which is being determined here). Use the path to the current
/* FP:config.rs-1355 */             // object file.
/* FP:config.rs-1356 */             (SplitDebuginfo::Packed | SplitDebuginfo::Unpacked, SplitDwarfKind::Single) => {
/* FP:config.rs-1357 */                 Some(obj_out)
/* FP:config.rs-1358 */             }
/* FP:config.rs-1359 */             // Split mode emits the DWARF into a different file, use that path.
/* FP:config.rs-1360 */             (SplitDebuginfo::Packed | SplitDebuginfo::Unpacked, SplitDwarfKind::Split) => {
/* FP:config.rs-1361 */                 Some(dwo_out)
/* FP:config.rs-1362 */             }
/* FP:config.rs-1363 */         }
/* FP:config.rs-1364 */     }
/* FP:config.rs-1365 */ }
/* FP:config.rs-1366 */ 
/* FP:config.rs-1367 */ bitflags::bitflags! {
/* FP:config.rs-1368 */     /// Scopes used to determined if it need to apply to --remap-path-prefix
/* FP:config.rs-1369 */     #[derive(Clone, Copy, PartialEq, Eq, Hash)]
/* FP:config.rs-1370 */     pub struct RemapPathScopeComponents: u8 {
/* FP:config.rs-1371 */         /// Apply remappings to the expansion of std::file!() macro
/* FP:config.rs-1372 */         const MACRO = 1 << 0;
/* FP:config.rs-1373 */         /// Apply remappings to printed compiler diagnostics
/* FP:config.rs-1374 */         const DIAGNOSTICS = 1 << 1;
/* FP:config.rs-1375 */         /// Apply remappings to debug information
/* FP:config.rs-1376 */         const DEBUGINFO = 1 << 3;
/* FP:config.rs-1377 */ 
/* FP:config.rs-1378 */         /// An alias for `macro` and `debuginfo`. This ensures all paths in compiled
/* FP:config.rs-1379 */         /// executables or libraries are remapped but not elsewhere.
/* FP:config.rs-1380 */         const OBJECT = Self::MACRO.bits() | Self::DEBUGINFO.bits();
/* FP:config.rs-1381 */     }
/* FP:config.rs-1382 */ }
/* FP:config.rs-1383 */ 
/* FP:config.rs-1384 */ #[derive(Clone, Debug)]
/* FP:config.rs-1385 */ pub struct Sysroot {
/* FP:config.rs-1386 */     pub explicit: Option<PathBuf>,
/* FP:config.rs-1387 */     pub default: PathBuf,
/* FP:config.rs-1388 */ }
/* FP:config.rs-1389 */ 
/* FP:config.rs-1390 */ impl Sysroot {
/* FP:config.rs-1391 */     pub fn new(explicit: Option<PathBuf>) -> Sysroot {
/* FP:config.rs-1392 */         Sysroot { explicit, default: filesearch::default_sysroot() }
/* FP:config.rs-1393 */     }
/* FP:config.rs-1394 */ 
/* FP:config.rs-1395 */     /// Return explicit sysroot if it was passed with `--sysroot`, or default sysroot otherwise.
/* FP:config.rs-1396 */     pub fn path(&self) -> &Path {
/* FP:config.rs-1397 */         self.explicit.as_deref().unwrap_or(&self.default)
/* FP:config.rs-1398 */     }
/* FP:config.rs-1399 */ 
/* FP:config.rs-1400 */     /// Returns both explicit sysroot if it was passed with `--sysroot` and the default sysroot.
/* FP:config.rs-1401 */     pub fn all_paths(&self) -> impl Iterator<Item = &Path> {
/* FP:config.rs-1402 */         self.explicit.as_deref().into_iter().chain(iter::once(&*self.default))
/* FP:config.rs-1403 */     }
/* FP:config.rs-1404 */ }
/* FP:config.rs-1405 */ 
/* FP:config.rs-1406 */ pub fn host_tuple() -> &'static str {
/* FP:config.rs-1407 */     // Get the host triple out of the build environment. This ensures that our
/* FP:config.rs-1408 */     // idea of the host triple is the same as for the set of libraries we've
/* FP:config.rs-1409 */     // actually built. We can't just take LLVM's host triple because they
/* FP:config.rs-1410 */     // normalize all ix86 architectures to i386.
/* FP:config.rs-1411 */     //
/* FP:config.rs-1412 */     // Instead of grabbing the host triple (for the current host), we grab (at
/* FP:config.rs-1413 */     // compile time) the target triple that this rustc is built with and
/* FP:config.rs-1414 */     // calling that (at runtime) the host triple.
/* FP:config.rs-1415 */     (option_env!("CFG_COMPILER_HOST_TRIPLE")).expect("CFG_COMPILER_HOST_TRIPLE")
/* FP:config.rs-1416 */ }
/* FP:config.rs-1417 */ 
/* FP:config.rs-1418 */ fn file_path_mapping(
/* FP:config.rs-1419 */     remap_path_prefix: Vec<(PathBuf, PathBuf)>,
/* FP:config.rs-1420 */     unstable_opts: &UnstableOptions,
/* FP:config.rs-1421 */ ) -> FilePathMapping {
/* FP:config.rs-1422 */     FilePathMapping::new(
/* FP:config.rs-1423 */         remap_path_prefix.clone(),
/* FP:config.rs-1424 */         if unstable_opts.remap_path_scope.contains(RemapPathScopeComponents::DIAGNOSTICS)
/* FP:config.rs-1425 */             && !remap_path_prefix.is_empty()
/* FP:config.rs-1426 */         {
/* FP:config.rs-1427 */             FileNameDisplayPreference::Remapped
/* FP:config.rs-1428 */         } else {
/* FP:config.rs-1429 */             FileNameDisplayPreference::Local
/* FP:config.rs-1430 */         },
/* FP:config.rs-1431 */         if unstable_opts.remap_path_scope.is_all() {
/* FP:config.rs-1432 */             FileNameEmbeddablePreference::RemappedOnly
/* FP:config.rs-1433 */         } else {
/* FP:config.rs-1434 */             FileNameEmbeddablePreference::LocalAndRemapped
/* FP:config.rs-1435 */         },
/* FP:config.rs-1436 */     )
/* FP:config.rs-1437 */ }
/* FP:config.rs-1438 */ 
/* FP:config.rs-1439 */ impl Default for Options {
/* FP:config.rs-1440 */     fn default() -> Options {
/* FP:config.rs-1441 */         Options {
/* FP:config.rs-1442 */             assert_incr_state: None,
/* FP:config.rs-1443 */             crate_types: Vec::new(),
/* FP:config.rs-1444 */             optimize: OptLevel::No,
/* FP:config.rs-1445 */             debuginfo: DebugInfo::None,
/* FP:config.rs-1446 */             debuginfo_compression: DebugInfoCompression::None,
/* FP:config.rs-1447 */             lint_opts: Vec::new(),
/* FP:config.rs-1448 */             lint_cap: None,
/* FP:config.rs-1449 */             describe_lints: false,
/* FP:config.rs-1450 */             output_types: OutputTypes(BTreeMap::new()),
/* FP:config.rs-1451 */             search_paths: vec![],
/* FP:config.rs-1452 */             sysroot: Sysroot::new(None),
/* FP:config.rs-1453 */             target_triple: TargetTuple::from_tuple(host_tuple()),
/* FP:config.rs-1454 */             test: false,
/* FP:config.rs-1455 */             incremental: None,
/* FP:config.rs-1456 */             untracked_state_hash: Default::default(),
/* FP:config.rs-1457 */             unstable_opts: Default::default(),
/* FP:config.rs-1458 */             prints: Vec::new(),
/* FP:config.rs-1459 */             cg: Default::default(),
/* FP:config.rs-1460 */             error_format: ErrorOutputType::default(),
/* FP:config.rs-1461 */             diagnostic_width: None,
/* FP:config.rs-1462 */             externs: Externs(BTreeMap::new()),
/* FP:config.rs-1463 */             crate_name: None,
/* FP:config.rs-1464 */             libs: Vec::new(),
/* FP:config.rs-1465 */             unstable_features: UnstableFeatures::Disallow,
/* FP:config.rs-1466 */             debug_assertions: true,
/* FP:config.rs-1467 */             actually_rustdoc: false,
/* FP:config.rs-1468 */             resolve_doc_links: ResolveDocLinks::None,
/* FP:config.rs-1469 */             trimmed_def_paths: false,
/* FP:config.rs-1470 */             cli_forced_codegen_units: None,
/* FP:config.rs-1471 */             cli_forced_local_thinlto_off: false,
/* FP:config.rs-1472 */             remap_path_prefix: Vec::new(),
/* FP:config.rs-1473 */             real_rust_source_base_dir: None,
/* FP:config.rs-1474 */             real_rustc_dev_source_base_dir: None,
/* FP:config.rs-1475 */             edition: DEFAULT_EDITION,
/* FP:config.rs-1476 */             json_artifact_notifications: false,
/* FP:config.rs-1477 */             json_timings: false,
/* FP:config.rs-1478 */             json_unused_externs: JsonUnusedExterns::No,
/* FP:config.rs-1479 */             json_future_incompat: false,
/* FP:config.rs-1480 */             pretty: None,
/* FP:config.rs-1481 */             working_dir: RealFileName::LocalPath(std::env::current_dir().unwrap()),
/* FP:config.rs-1482 */             color: ColorConfig::Auto,
/* FP:config.rs-1483 */             logical_env: FxIndexMap::default(),
/* FP:config.rs-1484 */             verbose: false,
/* FP:config.rs-1485 */             target_modifiers: BTreeMap::default(),
/* FP:config.rs-1486 */         }
/* FP:config.rs-1487 */     }
/* FP:config.rs-1488 */ }
/* FP:config.rs-1489 */ 
/* FP:config.rs-1490 */ impl Options {
/* FP:config.rs-1491 */     /// Returns `true` if there is a reason to build the dep graph.
/* FP:config.rs-1492 */     pub fn build_dep_graph(&self) -> bool {
/* FP:config.rs-1493 */         self.incremental.is_some()
/* FP:config.rs-1494 */             || self.unstable_opts.dump_dep_graph
/* FP:config.rs-1495 */             || self.unstable_opts.query_dep_graph
/* FP:config.rs-1496 */     }
/* FP:config.rs-1497 */ 
/* FP:config.rs-1498 */     pub fn file_path_mapping(&self) -> FilePathMapping {
/* FP:config.rs-1499 */         file_path_mapping(self.remap_path_prefix.clone(), &self.unstable_opts)
/* FP:config.rs-1500 */     }
/* FP:config.rs-1501 */ 
/* FP:config.rs-1502 */     /// Returns `true` if there will be an output file generated.
/* FP:config.rs-1503 */     pub fn will_create_output_file(&self) -> bool {
/* FP:config.rs-1504 */         !self.unstable_opts.parse_crate_root_only && // The file is just being parsed
/* FP:config.rs-1505 */             self.unstable_opts.ls.is_empty() // The file is just being queried
/* FP:config.rs-1506 */     }
/* FP:config.rs-1507 */ 
/* FP:config.rs-1508 */     #[inline]
/* FP:config.rs-1509 */     pub fn share_generics(&self) -> bool {
/* FP:config.rs-1510 */         match self.unstable_opts.share_generics {
/* FP:config.rs-1511 */             Some(setting) => setting,
/* FP:config.rs-1512 */             None => match self.optimize {
/* FP:config.rs-1513 */                 OptLevel::No | OptLevel::Less | OptLevel::Size | OptLevel::SizeMin => true,
/* FP:config.rs-1514 */                 OptLevel::More | OptLevel::Aggressive => false,
/* FP:config.rs-1515 */             },
/* FP:config.rs-1516 */         }
/* FP:config.rs-1517 */     }
/* FP:config.rs-1518 */ 
/* FP:config.rs-1519 */     pub fn get_symbol_mangling_version(&self) -> SymbolManglingVersion {
/* FP:config.rs-1520 */         self.cg.symbol_mangling_version.unwrap_or(SymbolManglingVersion::Legacy)
/* FP:config.rs-1521 */     }
/* FP:config.rs-1522 */ }
/* FP:config.rs-1523 */ 
/* FP:config.rs-1524 */ impl UnstableOptions {
/* FP:config.rs-1525 */     pub fn dcx_flags(&self, can_emit_warnings: bool) -> DiagCtxtFlags {
/* FP:config.rs-1526 */         DiagCtxtFlags {
/* FP:config.rs-1527 */             can_emit_warnings,
/* FP:config.rs-1528 */             treat_err_as_bug: self.treat_err_as_bug,
/* FP:config.rs-1529 */             eagerly_emit_delayed_bugs: self.eagerly_emit_delayed_bugs,
/* FP:config.rs-1530 */             macro_backtrace: self.macro_backtrace,
/* FP:config.rs-1531 */             deduplicate_diagnostics: self.deduplicate_diagnostics,
/* FP:config.rs-1532 */             track_diagnostics: self.track_diagnostics,
/* FP:config.rs-1533 */         }
/* FP:config.rs-1534 */     }
/* FP:config.rs-1535 */ 
/* FP:config.rs-1536 */     pub fn src_hash_algorithm(&self, target: &Target) -> SourceFileHashAlgorithm {
/* FP:config.rs-1537 */         self.src_hash_algorithm.unwrap_or_else(|| {
/* FP:config.rs-1538 */             if target.is_like_msvc {
/* FP:config.rs-1539 */                 SourceFileHashAlgorithm::Sha256
/* FP:config.rs-1540 */             } else {
/* FP:config.rs-1541 */                 SourceFileHashAlgorithm::Md5
/* FP:config.rs-1542 */             }
/* FP:config.rs-1543 */         })
/* FP:config.rs-1544 */     }
/* FP:config.rs-1545 */ 
/* FP:config.rs-1546 */     pub fn checksum_hash_algorithm(&self) -> Option<SourceFileHashAlgorithm> {
/* FP:config.rs-1547 */         self.checksum_hash_algorithm
/* FP:config.rs-1548 */     }
/* FP:config.rs-1549 */ }
/* FP:config.rs-1550 */ 
/* FP:config.rs-1551 */ // The type of entry function, so users can have their own entry functions
/* FP:config.rs-1552 */ #[derive(Copy, Clone, PartialEq, Hash, Debug, HashStable_Generic)]
/* FP:config.rs-1553 */ pub enum EntryFnType {
/* FP:config.rs-1554 */     Main {
/* FP:config.rs-1555 */         /// Specifies what to do with `SIGPIPE` before calling `fn main()`.
/* FP:config.rs-1556 */         ///
/* FP:config.rs-1557 */         /// What values that are valid and what they mean must be in sync
/* FP:config.rs-1558 */         /// across rustc and libstd, but we don't want it public in libstd,
/* FP:config.rs-1559 */         /// so we take a bit of an unusual approach with simple constants
/* FP:config.rs-1560 */         /// and an `include!()`.
/* FP:config.rs-1561 */         sigpipe: u8,
/* FP:config.rs-1562 */     },
/* FP:config.rs-1563 */ }
/* FP:config.rs-1564 */ 
/* FP:config.rs-1565 */ #[derive(Copy, PartialEq, PartialOrd, Clone, Ord, Eq, Hash, Debug, Encodable, Decodable)]
/* FP:config.rs-1566 */ #[derive(HashStable_Generic)]
/* FP:config.rs-1567 */ pub enum CrateType {
/* FP:config.rs-1568 */     Executable,
/* FP:config.rs-1569 */     Dylib,
/* FP:config.rs-1570 */     Rlib,
/* FP:config.rs-1571 */     Staticlib,
/* FP:config.rs-1572 */     Cdylib,
/* FP:config.rs-1573 */     ProcMacro,
/* FP:config.rs-1574 */     Sdylib,
/* FP:config.rs-1575 */ }
/* FP:config.rs-1576 */ 
/* FP:config.rs-1577 */ impl CrateType {
/* FP:config.rs-1578 */     pub fn has_metadata(self) -> bool {
/* FP:config.rs-1579 */         match self {
/* FP:config.rs-1580 */             CrateType::Rlib | CrateType::Dylib | CrateType::ProcMacro => true,
/* FP:config.rs-1581 */             CrateType::Executable
/* FP:config.rs-1582 */             | CrateType::Cdylib
/* FP:config.rs-1583 */             | CrateType::Staticlib
/* FP:config.rs-1584 */             | CrateType::Sdylib => false,
/* FP:config.rs-1585 */         }
/* FP:config.rs-1586 */     }
/* FP:config.rs-1587 */ }
/* FP:config.rs-1588 */ 
/* FP:config.rs-1589 */ #[derive(Clone, Hash, Debug, PartialEq, Eq)]
/* FP:config.rs-1590 */ pub enum Passes {
/* FP:config.rs-1591 */     Some(Vec<String>),
/* FP:config.rs-1592 */     All,
/* FP:config.rs-1593 */ }
/* FP:config.rs-1594 */ 
/* FP:config.rs-1595 */ impl Passes {
/* FP:config.rs-1596 */     fn is_empty(&self) -> bool {
/* FP:config.rs-1597 */         match *self {
/* FP:config.rs-1598 */             Passes::Some(ref v) => v.is_empty(),
/* FP:config.rs-1599 */             Passes::All => false,
/* FP:config.rs-1600 */         }
/* FP:config.rs-1601 */     }
/* FP:config.rs-1602 */ 
/* FP:config.rs-1603 */     pub(crate) fn extend(&mut self, passes: impl IntoIterator<Item = String>) {
/* FP:config.rs-1604 */         match *self {
/* FP:config.rs-1605 */             Passes::Some(ref mut v) => v.extend(passes),
/* FP:config.rs-1606 */             Passes::All => {}
/* FP:config.rs-1607 */         }
/* FP:config.rs-1608 */     }
/* FP:config.rs-1609 */ }
/* FP:config.rs-1610 */ 
/* FP:config.rs-1611 */ #[derive(Clone, Copy, Hash, Debug, PartialEq)]
/* FP:config.rs-1612 */ pub enum PAuthKey {
/* FP:config.rs-1613 */     A,
/* FP:config.rs-1614 */     B,
/* FP:config.rs-1615 */ }
/* FP:config.rs-1616 */ 
/* FP:config.rs-1617 */ #[derive(Clone, Copy, Hash, Debug, PartialEq)]
/* FP:config.rs-1618 */ pub struct PacRet {
/* FP:config.rs-1619 */     pub leaf: bool,
/* FP:config.rs-1620 */     pub pc: bool,
/* FP:config.rs-1621 */     pub key: PAuthKey,
/* FP:config.rs-1622 */ }
/* FP:config.rs-1623 */ 
/* FP:config.rs-1624 */ #[derive(Clone, Copy, Hash, Debug, PartialEq, Default)]
/* FP:config.rs-1625 */ pub struct BranchProtection {
/* FP:config.rs-1626 */     pub bti: bool,
/* FP:config.rs-1627 */     pub pac_ret: Option<PacRet>,
/* FP:config.rs-1628 */ }
/* FP:config.rs-1629 */ 
/* FP:config.rs-1630 */ pub(crate) const fn default_lib_output() -> CrateType {
/* FP:config.rs-1631 */     CrateType::Rlib
/* FP:config.rs-1632 */ }
/* FP:config.rs-1633 */ 
/* FP:config.rs-1634 */ pub fn build_configuration(sess: &Session, mut user_cfg: Cfg) -> Cfg {
/* FP:config.rs-1635 */     // First disallow some configuration given on the command line
/* FP:config.rs-1636 */     cfg::disallow_cfgs(sess, &user_cfg);
/* FP:config.rs-1637 */ 
/* FP:config.rs-1638 */     // Then combine the configuration requested by the session (command line) with
/* FP:config.rs-1639 */     // some default and generated configuration items.
/* FP:config.rs-1640 */     user_cfg.extend(cfg::default_configuration(sess));
/* FP:config.rs-1641 */     user_cfg
/* FP:config.rs-1642 */ }
/* FP:config.rs-1643 */ 
/* FP:config.rs-1644 */ pub fn build_target_config(
/* FP:config.rs-1645 */     early_dcx: &EarlyDiagCtxt,
/* FP:config.rs-1646 */     target: &TargetTuple,
/* FP:config.rs-1647 */     sysroot: &Path,
/* FP:config.rs-1648 */ ) -> Target {
/* FP:config.rs-1649 */     match Target::search(target, sysroot) {
/* FP:config.rs-1650 */         Ok((target, warnings)) => {
/* FP:config.rs-1651 */             for warning in warnings.warning_messages() {
/* FP:config.rs-1652 */                 early_dcx.early_warn(warning)
/* FP:config.rs-1653 */             }
/* FP:config.rs-1654 */ 
/* FP:config.rs-1655 */             if !matches!(target.pointer_width, 16 | 32 | 64) {
/* FP:config.rs-1656 */                 early_dcx.early_fatal(format!(
/* FP:config.rs-1657 */                     "target specification was invalid: unrecognized target-pointer-width {}",
/* FP:config.rs-1658 */                     target.pointer_width
/* FP:config.rs-1659 */                 ))
/* FP:config.rs-1660 */             }
/* FP:config.rs-1661 */             target
/* FP:config.rs-1662 */         }
/* FP:config.rs-1663 */         Err(e) => {
/* FP:config.rs-1664 */             let mut err =
/* FP:config.rs-1665 */                 early_dcx.early_struct_fatal(format!("error loading target specification: {e}"));
/* FP:config.rs-1666 */             err.help("run `rustc --print target-list` for a list of built-in targets");
/* FP:config.rs-1667 */             err.emit();
/* FP:config.rs-1668 */         }
/* FP:config.rs-1669 */     }
/* FP:config.rs-1670 */ }
/* FP:config.rs-1671 */ 
/* FP:config.rs-1672 */ #[derive(Copy, Clone, PartialEq, Eq, Debug)]
/* FP:config.rs-1673 */ pub enum OptionStability {
/* FP:config.rs-1674 */     Stable,
/* FP:config.rs-1675 */     Unstable,
/* FP:config.rs-1676 */ }
/* FP:config.rs-1677 */ 
/* FP:config.rs-1678 */ #[derive(Copy, Clone, PartialEq, Eq, Debug)]
/* FP:config.rs-1679 */ pub enum OptionKind {
/* FP:config.rs-1680 */     /// An option that takes a value, and cannot appear more than once (e.g. `--out-dir`).
/* FP:config.rs-1681 */     ///
/* FP:config.rs-1682 */     /// Corresponds to [`getopts::Options::optopt`].
/* FP:config.rs-1683 */     Opt,
/* FP:config.rs-1684 */ 
/* FP:config.rs-1685 */     /// An option that takes a value, and can appear multiple times (e.g. `--emit`).
/* FP:config.rs-1686 */     ///
/* FP:config.rs-1687 */     /// Corresponds to [`getopts::Options::optmulti`].
/* FP:config.rs-1688 */     Multi,
/* FP:config.rs-1689 */ 
/* FP:config.rs-1690 */     /// An option that does not take a value, and cannot appear more than once (e.g. `--help`).
/* FP:config.rs-1691 */     ///
/* FP:config.rs-1692 */     /// Corresponds to [`getopts::Options::optflag`].
/* FP:config.rs-1693 */     /// The `hint` string must be empty.
/* FP:config.rs-1694 */     Flag,
/* FP:config.rs-1695 */ 
/* FP:config.rs-1696 */     /// An option that does not take a value, and can appear multiple times (e.g. `-O`).
/* FP:config.rs-1697 */     ///
/* FP:config.rs-1698 */     /// Corresponds to [`getopts::Options::optflagmulti`].
/* FP:config.rs-1699 */     /// The `hint` string must be empty.
/* FP:config.rs-1700 */     FlagMulti,
/* FP:config.rs-1701 */ }
/* FP:config.rs-1702 */ 
/* FP:config.rs-1703 */ pub struct RustcOptGroup {
/* FP:config.rs-1704 */     /// The "primary" name for this option. Normally equal to `long_name`,
/* FP:config.rs-1705 */     /// except for options that don't have a long name, in which case
/* FP:config.rs-1706 */     /// `short_name` is used.
/* FP:config.rs-1707 */     ///
/* FP:config.rs-1708 */     /// This is needed when interacting with `getopts` in some situations,
/* FP:config.rs-1709 */     /// because if an option has both forms, that library treats the long name
/* FP:config.rs-1710 */     /// as primary and the short name as an alias.
/* FP:config.rs-1711 */     pub name: &'static str,
/* FP:config.rs-1712 */     stability: OptionStability,
/* FP:config.rs-1713 */     kind: OptionKind,
/* FP:config.rs-1714 */ 
/* FP:config.rs-1715 */     short_name: &'static str,
/* FP:config.rs-1716 */     long_name: &'static str,
/* FP:config.rs-1717 */     desc: &'static str,
/* FP:config.rs-1718 */     value_hint: &'static str,
/* FP:config.rs-1719 */ 
/* FP:config.rs-1720 */     /// If true, this option should not be printed by `rustc --help`, but
/* FP:config.rs-1721 */     /// should still be printed by `rustc --help -v`.
/* FP:config.rs-1722 */     pub is_verbose_help_only: bool,
/* FP:config.rs-1723 */ }
/* FP:config.rs-1724 */ 
/* FP:config.rs-1725 */ impl RustcOptGroup {
/* FP:config.rs-1726 */     pub fn is_stable(&self) -> bool {
/* FP:config.rs-1727 */         self.stability == OptionStability::Stable
/* FP:config.rs-1728 */     }
/* FP:config.rs-1729 */ 
/* FP:config.rs-1730 */     pub fn apply(&self, options: &mut getopts::Options) {
/* FP:config.rs-1731 */         let &Self { short_name, long_name, desc, value_hint, .. } = self;
/* FP:config.rs-1732 */         match self.kind {
/* FP:config.rs-1733 */             OptionKind::Opt => options.optopt(short_name, long_name, desc, value_hint),
/* FP:config.rs-1734 */             OptionKind::Multi => options.optmulti(short_name, long_name, desc, value_hint),
/* FP:config.rs-1735 */             OptionKind::Flag => options.optflag(short_name, long_name, desc),
/* FP:config.rs-1736 */             OptionKind::FlagMulti => options.optflagmulti(short_name, long_name, desc),
/* FP:config.rs-1737 */         };
/* FP:config.rs-1738 */     }
/* FP:config.rs-1739 */ 
/* FP:config.rs-1740 */     /// This is for diagnostics-only.
/* FP:config.rs-1741 */     pub fn long_name(&self) -> &str {
/* FP:config.rs-1742 */         self.long_name
/* FP:config.rs-1743 */     }
/* FP:config.rs-1744 */ }
/* FP:config.rs-1745 */ 
/* FP:config.rs-1746 */ pub fn make_opt(
/* FP:config.rs-1747 */     stability: OptionStability,
/* FP:config.rs-1748 */     kind: OptionKind,
/* FP:config.rs-1749 */     short_name: &'static str,
/* FP:config.rs-1750 */     long_name: &'static str,
/* FP:config.rs-1751 */     desc: &'static str,
/* FP:config.rs-1752 */     value_hint: &'static str,
/* FP:config.rs-1753 */ ) -> RustcOptGroup {
/* FP:config.rs-1754 */     // "Flag" options don't have a value, and therefore don't have a value hint.
/* FP:config.rs-1755 */     match kind {
/* FP:config.rs-1756 */         OptionKind::Opt | OptionKind::Multi => {}
/* FP:config.rs-1757 */         OptionKind::Flag | OptionKind::FlagMulti => assert_eq!(value_hint, ""),
/* FP:config.rs-1758 */     }
/* FP:config.rs-1759 */     RustcOptGroup {
/* FP:config.rs-1760 */         name: cmp::max_by_key(short_name, long_name, |s| s.len()),
/* FP:config.rs-1761 */         stability,
/* FP:config.rs-1762 */         kind,
/* FP:config.rs-1763 */         short_name,
/* FP:config.rs-1764 */         long_name,
/* FP:config.rs-1765 */         desc,
/* FP:config.rs-1766 */         value_hint,
/* FP:config.rs-1767 */         is_verbose_help_only: false,
/* FP:config.rs-1768 */     }
/* FP:config.rs-1769 */ }
/* FP:config.rs-1770 */ 
/* FP:config.rs-1771 */ static EDITION_STRING: LazyLock<String> = LazyLock::new(|| {
/* FP:config.rs-1772 */     format!(
/* FP:config.rs-1773 */         "Specify which edition of the compiler to use when compiling code. \
/* FP:config.rs-1774 */ The default is {DEFAULT_EDITION} and the latest stable edition is {LATEST_STABLE_EDITION}."
/* FP:config.rs-1775 */     )
/* FP:config.rs-1776 */ });
/* FP:config.rs-1777 */ 
/* FP:config.rs-1778 */ static PRINT_HELP: LazyLock<String> = LazyLock::new(|| {
/* FP:config.rs-1779 */     format!(
/* FP:config.rs-1780 */         "Compiler information to print on stdout (or to a file)\n\
/* FP:config.rs-1781 */         INFO may be one of <{}>.",
/* FP:config.rs-1782 */         PRINT_KINDS.iter().map(|(name, _)| format!("{name}")).collect::<Vec<_>>().join("|")
/* FP:config.rs-1783 */     )
/* FP:config.rs-1784 */ });
/* FP:config.rs-1785 */ 
/* FP:config.rs-1786 */ static EMIT_HELP: LazyLock<String> = LazyLock::new(|| {
/* FP:config.rs-1787 */     let mut result =
/* FP:config.rs-1788 */         String::from("Comma separated list of types of output for the compiler to emit.\n");
/* FP:config.rs-1789 */     result.push_str("Each TYPE has the default FILE name:\n");
/* FP:config.rs-1790 */ 
/* FP:config.rs-1791 */     for output in OutputType::iter_all() {
/* FP:config.rs-1792 */         result.push_str(&format!("*  {} - {}\n", output.shorthand(), output.default_filename()));
/* FP:config.rs-1793 */     }
/* FP:config.rs-1794 */ 
/* FP:config.rs-1795 */     result
/* FP:config.rs-1796 */ });
/* FP:config.rs-1797 */ 
/* FP:config.rs-1798 */ /// Returns all rustc command line options, including metadata for
/* FP:config.rs-1799 */ /// each option, such as whether the option is stable.
/* FP:config.rs-1800 */ ///
/* FP:config.rs-1801 */ /// # Option style guidelines
/* FP:config.rs-1802 */ ///
/* FP:config.rs-1803 */ /// - `<param>`: Indicates a required parameter
/* FP:config.rs-1804 */ /// - `[param]`: Indicates an optional parameter
/* FP:config.rs-1805 */ /// - `|`: Indicates a mutually exclusive option
/* FP:config.rs-1806 */ /// - `*`: a list element with description
/* FP:config.rs-1807 */ pub fn rustc_optgroups() -> Vec<RustcOptGroup> {
/* FP:config.rs-1808 */     use OptionKind::{Flag, FlagMulti, Multi, Opt};
/* FP:config.rs-1809 */     use OptionStability::{Stable, Unstable};
/* FP:config.rs-1810 */ 
/* FP:config.rs-1811 */     use self::make_opt as opt;
/* FP:config.rs-1812 */ 
/* FP:config.rs-1813 */     let mut options = vec![
/* FP:config.rs-1814 */         opt(Stable, Flag, "h", "help", "Display this message", ""),
/* FP:config.rs-1815 */         opt(
/* FP:config.rs-1816 */             Stable,
/* FP:config.rs-1817 */             Multi,
/* FP:config.rs-1818 */             "",
/* FP:config.rs-1819 */             "cfg",
/* FP:config.rs-1820 */             "Configure the compilation environment.\n\
/* FP:config.rs-1821 */                 SPEC supports the syntax `<NAME>[=\"<VALUE>\"]`.",
/* FP:config.rs-1822 */             "<SPEC>",
/* FP:config.rs-1823 */         ),
/* FP:config.rs-1824 */         opt(Stable, Multi, "", "check-cfg", "Provide list of expected cfgs for checking", "<SPEC>"),
/* FP:config.rs-1825 */         opt(
/* FP:config.rs-1826 */             Stable,
/* FP:config.rs-1827 */             Multi,
/* FP:config.rs-1828 */             "L",
/* FP:config.rs-1829 */             "",
/* FP:config.rs-1830 */             "Add a directory to the library search path. \
/* FP:config.rs-1831 */                 The optional KIND can be one of <dependency|crate|native|framework|all> (default: all).",
/* FP:config.rs-1832 */             "[<KIND>=]<PATH>",
/* FP:config.rs-1833 */         ),
/* FP:config.rs-1834 */         opt(
/* FP:config.rs-1835 */             Stable,
/* FP:config.rs-1836 */             Multi,
/* FP:config.rs-1837 */             "l",
/* FP:config.rs-1838 */             "",
/* FP:config.rs-1839 */             "Link the generated crate(s) to the specified native\n\
/* FP:config.rs-1840 */                 library NAME. The optional KIND can be one of\n\
/* FP:config.rs-1841 */                 <static|framework|dylib> (default: dylib).\n\
/* FP:config.rs-1842 */                 Optional comma separated MODIFIERS\n\
/* FP:config.rs-1843 */                 <bundle|verbatim|whole-archive|as-needed>\n\
/* FP:config.rs-1844 */                 may be specified each with a prefix of either '+' to\n\
/* FP:config.rs-1845 */                 enable or '-' to disable.",
/* FP:config.rs-1846 */             "[<KIND>[:<MODIFIERS>]=]<NAME>[:<RENAME>]",
/* FP:config.rs-1847 */         ),
/* FP:config.rs-1848 */         make_crate_type_option(),
/* FP:config.rs-1849 */         opt(Stable, Opt, "", "crate-name", "Specify the name of the crate being built", "<NAME>"),
/* FP:config.rs-1850 */         opt(Stable, Opt, "", "edition", &EDITION_STRING, EDITION_NAME_LIST),
/* FP:config.rs-1851 */         opt(Stable, Multi, "", "emit", &EMIT_HELP, "<TYPE>[=<FILE>]"),
/* FP:config.rs-1852 */         opt(Stable, Multi, "", "print", &PRINT_HELP, "<INFO>[=<FILE>]"),
/* FP:config.rs-1853 */         opt(Stable, FlagMulti, "g", "", "Equivalent to -C debuginfo=2", ""),
/* FP:config.rs-1854 */         opt(Stable, FlagMulti, "O", "", "Equivalent to -C opt-level=3", ""),
/* FP:config.rs-1855 */         opt(Stable, Opt, "o", "", "Write output to FILENAME", "<FILENAME>"),
/* FP:config.rs-1856 */         opt(Stable, Opt, "", "out-dir", "Write output to compiler-chosen filename in DIR", "<DIR>"),
/* FP:config.rs-1857 */         opt(
/* FP:config.rs-1858 */             Stable,
/* FP:config.rs-1859 */             Opt,
/* FP:config.rs-1860 */             "",
/* FP:config.rs-1861 */             "explain",
/* FP:config.rs-1862 */             "Provide a detailed explanation of an error message",
/* FP:config.rs-1863 */             "<OPT>",
/* FP:config.rs-1864 */         ),
/* FP:config.rs-1865 */         opt(Stable, Flag, "", "test", "Build a test harness", ""),
/* FP:config.rs-1866 */         opt(Stable, Opt, "", "target", "Target triple for which the code is compiled", "<TARGET>"),
/* FP:config.rs-1867 */         opt(Stable, Multi, "A", "allow", "Set lint allowed", "<LINT>"),
/* FP:config.rs-1868 */         opt(Stable, Multi, "W", "warn", "Set lint warnings", "<LINT>"),
/* FP:config.rs-1869 */         opt(Stable, Multi, "", "force-warn", "Set lint force-warn", "<LINT>"),
/* FP:config.rs-1870 */         opt(Stable, Multi, "D", "deny", "Set lint denied", "<LINT>"),
/* FP:config.rs-1871 */         opt(Stable, Multi, "F", "forbid", "Set lint forbidden", "<LINT>"),
/* FP:config.rs-1872 */         opt(
/* FP:config.rs-1873 */             Stable,
/* FP:config.rs-1874 */             Multi,
/* FP:config.rs-1875 */             "",
/* FP:config.rs-1876 */             "cap-lints",
/* FP:config.rs-1877 */             "Set the most restrictive lint level. More restrictive lints are capped at this level",
/* FP:config.rs-1878 */             "<LEVEL>",
/* FP:config.rs-1879 */         ),
/* FP:config.rs-1880 */         opt(Stable, Multi, "C", "codegen", "Set a codegen option", "<OPT>[=<VALUE>]"),
/* FP:config.rs-1881 */         opt(Stable, Flag, "V", "version", "Print version info and exit", ""),
/* FP:config.rs-1882 */         opt(Stable, Flag, "v", "verbose", "Use verbose output", ""),
/* FP:config.rs-1883 */     ];
/* FP:config.rs-1884 */ 
/* FP:config.rs-1885 */     // Options in this list are hidden from `rustc --help` by default, but are
/* FP:config.rs-1886 */     // shown by `rustc --help -v`.
/* FP:config.rs-1887 */     let verbose_only = [
/* FP:config.rs-1888 */         opt(
/* FP:config.rs-1889 */             Stable,
/* FP:config.rs-1890 */             Multi,
/* FP:config.rs-1891 */             "",
/* FP:config.rs-1892 */             "extern",
/* FP:config.rs-1893 */             "Specify where an external rust library is located",
/* FP:config.rs-1894 */             "<NAME>[=<PATH>]",
/* FP:config.rs-1895 */         ),
/* FP:config.rs-1896 */         opt(Stable, Opt, "", "sysroot", "Override the system root", "<PATH>"),
/* FP:config.rs-1897 */         opt(Unstable, Multi, "Z", "", "Set unstable / perma-unstable options", "<FLAG>"),
/* FP:config.rs-1898 */         opt(
/* FP:config.rs-1899 */             Stable,
/* FP:config.rs-1900 */             Opt,
/* FP:config.rs-1901 */             "",
/* FP:config.rs-1902 */             "error-format",
/* FP:config.rs-1903 */             "How errors and other messages are produced",
/* FP:config.rs-1904 */             "<human|json|short>",
/* FP:config.rs-1905 */         ),
/* FP:config.rs-1906 */         opt(Stable, Multi, "", "json", "Configure the JSON output of the compiler", "<CONFIG>"),
/* FP:config.rs-1907 */         opt(
/* FP:config.rs-1908 */             Stable,
/* FP:config.rs-1909 */             Opt,
/* FP:config.rs-1910 */             "",
/* FP:config.rs-1911 */             "color",
/* FP:config.rs-1912 */             "Configure coloring of output:
/* FP:config.rs-1913 */                 * auto   = colorize, if output goes to a tty (default);
/* FP:config.rs-1914 */                 * always = always colorize output;
/* FP:config.rs-1915 */                 * never  = never colorize output",
/* FP:config.rs-1916 */             "<auto|always|never>",
/* FP:config.rs-1917 */         ),
/* FP:config.rs-1918 */         opt(
/* FP:config.rs-1919 */             Stable,
/* FP:config.rs-1920 */             Opt,
/* FP:config.rs-1921 */             "",
/* FP:config.rs-1922 */             "diagnostic-width",
/* FP:config.rs-1923 */             "Inform rustc of the width of the output so that diagnostics can be truncated to fit",
/* FP:config.rs-1924 */             "<WIDTH>",
/* FP:config.rs-1925 */         ),
/* FP:config.rs-1926 */         opt(
/* FP:config.rs-1927 */             Stable,
/* FP:config.rs-1928 */             Multi,
/* FP:config.rs-1929 */             "",
/* FP:config.rs-1930 */             "remap-path-prefix",
/* FP:config.rs-1931 */             "Remap source names in all output (compiler messages and output files)",
/* FP:config.rs-1932 */             "<FROM>=<TO>",
/* FP:config.rs-1933 */         ),
/* FP:config.rs-1934 */         opt(Unstable, Multi, "", "env-set", "Inject an environment variable", "<VAR>=<VALUE>"),
/* FP:config.rs-1935 */     ];
/* FP:config.rs-1936 */     options.extend(verbose_only.into_iter().map(|mut opt| {
/* FP:config.rs-1937 */         opt.is_verbose_help_only = true;
/* FP:config.rs-1938 */         opt
/* FP:config.rs-1939 */     }));
/* FP:config.rs-1940 */ 
/* FP:config.rs-1941 */     options
/* FP:config.rs-1942 */ }
/* FP:config.rs-1943 */ 
/* FP:config.rs-1944 */ pub fn get_cmd_lint_options(
/* FP:config.rs-1945 */     early_dcx: &EarlyDiagCtxt,
/* FP:config.rs-1946 */     matches: &getopts::Matches,
/* FP:config.rs-1947 */ ) -> (Vec<(String, lint::Level)>, bool, Option<lint::Level>) {
/* FP:config.rs-1948 */     let mut lint_opts_with_position = vec![];
/* FP:config.rs-1949 */     let mut describe_lints = false;
/* FP:config.rs-1950 */ 
/* FP:config.rs-1951 */     for level in [lint::Allow, lint::Warn, lint::ForceWarn, lint::Deny, lint::Forbid] {
/* FP:config.rs-1952 */         for (arg_pos, lint_name) in matches.opt_strs_pos(level.as_str()) {
/* FP:config.rs-1953 */             if lint_name == "help" {
/* FP:config.rs-1954 */                 describe_lints = true;
/* FP:config.rs-1955 */             } else {
/* FP:config.rs-1956 */                 lint_opts_with_position.push((arg_pos, lint_name.replace('-', "_"), level));
/* FP:config.rs-1957 */             }
/* FP:config.rs-1958 */         }
/* FP:config.rs-1959 */     }
/* FP:config.rs-1960 */ 
/* FP:config.rs-1961 */     lint_opts_with_position.sort_by_key(|x| x.0);
/* FP:config.rs-1962 */     let lint_opts = lint_opts_with_position
/* FP:config.rs-1963 */         .iter()
/* FP:config.rs-1964 */         .cloned()
/* FP:config.rs-1965 */         .map(|(_, lint_name, level)| (lint_name, level))
/* FP:config.rs-1966 */         .collect();
/* FP:config.rs-1967 */ 
/* FP:config.rs-1968 */     let lint_cap = matches.opt_str("cap-lints").map(|cap| {
/* FP:config.rs-1969 */         lint::Level::from_str(&cap)
/* FP:config.rs-1970 */             .unwrap_or_else(|| early_dcx.early_fatal(format!("unknown lint level: `{cap}`")))
/* FP:config.rs-1971 */     });
/* FP:config.rs-1972 */ 
/* FP:config.rs-1973 */     (lint_opts, describe_lints, lint_cap)
/* FP:config.rs-1974 */ }
/* FP:config.rs-1975 */ 
/* FP:config.rs-1976 */ /// Parses the `--color` flag.
/* FP:config.rs-1977 */ pub fn parse_color(early_dcx: &EarlyDiagCtxt, matches: &getopts::Matches) -> ColorConfig {
/* FP:config.rs-1978 */     match matches.opt_str("color").as_deref() {
/* FP:config.rs-1979 */         Some("auto") => ColorConfig::Auto,
/* FP:config.rs-1980 */         Some("always") => ColorConfig::Always,
/* FP:config.rs-1981 */         Some("never") => ColorConfig::Never,
/* FP:config.rs-1982 */ 
/* FP:config.rs-1983 */         None => ColorConfig::Auto,
/* FP:config.rs-1984 */ 
/* FP:config.rs-1985 */         Some(arg) => early_dcx.early_fatal(format!(
/* FP:config.rs-1986 */             "argument for `--color` must be auto, \
/* FP:config.rs-1987 */                  always or never (instead was `{arg}`)"
/* FP:config.rs-1988 */         )),
/* FP:config.rs-1989 */     }
/* FP:config.rs-1990 */ }
/* FP:config.rs-1991 */ 
/* FP:config.rs-1992 */ /// Possible json config files
/* FP:config.rs-1993 */ pub struct JsonConfig {
/* FP:config.rs-1994 */     pub json_rendered: HumanReadableErrorType,
/* FP:config.rs-1995 */     pub json_color: ColorConfig,
/* FP:config.rs-1996 */     json_artifact_notifications: bool,
/* FP:config.rs-1997 */     /// Output start and end timestamps of several high-level compilation sections
/* FP:config.rs-1998 */     /// (frontend, backend, linker).
/* FP:config.rs-1999 */     json_timings: bool,
/* FP:config.rs-2000 */     pub json_unused_externs: JsonUnusedExterns,
/* FP:config.rs-2001 */     json_future_incompat: bool,
/* FP:config.rs-2002 */ }
/* FP:config.rs-2003 */ 
/* FP:config.rs-2004 */ /// Report unused externs in event stream
/* FP:config.rs-2005 */ #[derive(Copy, Clone)]
/* FP:config.rs-2006 */ pub enum JsonUnusedExterns {
/* FP:config.rs-2007 */     /// Do not
/* FP:config.rs-2008 */     No,
/* FP:config.rs-2009 */     /// Report, but do not exit with failure status for deny/forbid
/* FP:config.rs-2010 */     Silent,
/* FP:config.rs-2011 */     /// Report, and also exit with failure status for deny/forbid
/* FP:config.rs-2012 */     Loud,
/* FP:config.rs-2013 */ }
/* FP:config.rs-2014 */ 
/* FP:config.rs-2015 */ impl JsonUnusedExterns {
/* FP:config.rs-2016 */     pub fn is_enabled(&self) -> bool {
/* FP:config.rs-2017 */         match self {
/* FP:config.rs-2018 */             JsonUnusedExterns::No => false,
/* FP:config.rs-2019 */             JsonUnusedExterns::Loud | JsonUnusedExterns::Silent => true,
/* FP:config.rs-2020 */         }
/* FP:config.rs-2021 */     }
/* FP:config.rs-2022 */ 
/* FP:config.rs-2023 */     pub fn is_loud(&self) -> bool {
/* FP:config.rs-2024 */         match self {
/* FP:config.rs-2025 */             JsonUnusedExterns::No | JsonUnusedExterns::Silent => false,
/* FP:config.rs-2026 */             JsonUnusedExterns::Loud => true,
/* FP:config.rs-2027 */         }
/* FP:config.rs-2028 */     }
/* FP:config.rs-2029 */ }
/* FP:config.rs-2030 */ 
/* FP:config.rs-2031 */ /// Parse the `--json` flag.
/* FP:config.rs-2032 */ ///
/* FP:config.rs-2033 */ /// The first value returned is how to render JSON diagnostics, and the second
/* FP:config.rs-2034 */ /// is whether or not artifact notifications are enabled.
/* FP:config.rs-2035 */ pub fn parse_json(early_dcx: &EarlyDiagCtxt, matches: &getopts::Matches) -> JsonConfig {
/* FP:config.rs-2036 */     let mut json_rendered = HumanReadableErrorType::Default;
/* FP:config.rs-2037 */     let mut json_color = ColorConfig::Never;
/* FP:config.rs-2038 */     let mut json_artifact_notifications = false;
/* FP:config.rs-2039 */     let mut json_unused_externs = JsonUnusedExterns::No;
/* FP:config.rs-2040 */     let mut json_future_incompat = false;
/* FP:config.rs-2041 */     let mut json_timings = false;
/* FP:config.rs-2042 */     for option in matches.opt_strs("json") {
/* FP:config.rs-2043 */         // For now conservatively forbid `--color` with `--json` since `--json`
/* FP:config.rs-2044 */         // won't actually be emitting any colors and anything colorized is
/* FP:config.rs-2045 */         // embedded in a diagnostic message anyway.
/* FP:config.rs-2046 */         if matches.opt_str("color").is_some() {
/* FP:config.rs-2047 */             early_dcx.early_fatal("cannot specify the `--color` option with `--json`");
/* FP:config.rs-2048 */         }
/* FP:config.rs-2049 */ 
/* FP:config.rs-2050 */         for sub_option in option.split(',') {
/* FP:config.rs-2051 */             match sub_option {
/* FP:config.rs-2052 */                 "diagnostic-short" => json_rendered = HumanReadableErrorType::Short,
/* FP:config.rs-2053 */                 "diagnostic-unicode" => {
/* FP:config.rs-2054 */                     json_rendered = HumanReadableErrorType::Unicode;
/* FP:config.rs-2055 */                 }
/* FP:config.rs-2056 */                 "diagnostic-rendered-ansi" => json_color = ColorConfig::Always,
/* FP:config.rs-2057 */                 "artifacts" => json_artifact_notifications = true,
/* FP:config.rs-2058 */                 "timings" => json_timings = true,
/* FP:config.rs-2059 */                 "unused-externs" => json_unused_externs = JsonUnusedExterns::Loud,
/* FP:config.rs-2060 */                 "unused-externs-silent" => json_unused_externs = JsonUnusedExterns::Silent,
/* FP:config.rs-2061 */                 "future-incompat" => json_future_incompat = true,
/* FP:config.rs-2062 */                 s => early_dcx.early_fatal(format!("unknown `--json` option `{s}`")),
/* FP:config.rs-2063 */             }
/* FP:config.rs-2064 */         }
/* FP:config.rs-2065 */     }
/* FP:config.rs-2066 */ 
/* FP:config.rs-2067 */     JsonConfig {
/* FP:config.rs-2068 */         json_rendered,
/* FP:config.rs-2069 */         json_color,
/* FP:config.rs-2070 */         json_artifact_notifications,
/* FP:config.rs-2071 */         json_timings,
/* FP:config.rs-2072 */         json_unused_externs,
/* FP:config.rs-2073 */         json_future_incompat,
/* FP:config.rs-2074 */     }
/* FP:config.rs-2075 */ }
/* FP:config.rs-2076 */ 
/* FP:config.rs-2077 */ /// Parses the `--error-format` flag.
/* FP:config.rs-2078 */ pub fn parse_error_format(
/* FP:config.rs-2079 */     early_dcx: &mut EarlyDiagCtxt,
/* FP:config.rs-2080 */     matches: &getopts::Matches,
/* FP:config.rs-2081 */     color_config: ColorConfig,
/* FP:config.rs-2082 */     json_color: ColorConfig,
/* FP:config.rs-2083 */     json_rendered: HumanReadableErrorType,
/* FP:config.rs-2084 */ ) -> ErrorOutputType {
/* FP:config.rs-2085 */     // We need the `opts_present` check because the driver will send us Matches
/* FP:config.rs-2086 */     // with only stable options if no unstable options are used. Since error-format
/* FP:config.rs-2087 */     // is unstable, it will not be present. We have to use `opts_present` not
/* FP:config.rs-2088 */     // `opt_present` because the latter will panic.
/* FP:config.rs-2089 */     let error_format = if matches.opts_present(&["error-format".to_owned()]) {
/* FP:config.rs-2090 */         match matches.opt_str("error-format").as_deref() {
/* FP:config.rs-2091 */             None | Some("human") => ErrorOutputType::HumanReadable { color_config, .. },
/* FP:config.rs-2092 */             Some("human-annotate-rs") => ErrorOutputType::HumanReadable {
/* FP:config.rs-2093 */                 kind: HumanReadableErrorType::AnnotateSnippet,
/* FP:config.rs-2094 */                 color_config,
/* FP:config.rs-2095 */             },
/* FP:config.rs-2096 */             Some("json") => {
/* FP:config.rs-2097 */                 ErrorOutputType::Json { pretty: false, json_rendered, color_config: json_color }
/* FP:config.rs-2098 */             }
/* FP:config.rs-2099 */             Some("pretty-json") => {
/* FP:config.rs-2100 */                 ErrorOutputType::Json { pretty: true, json_rendered, color_config: json_color }
/* FP:config.rs-2101 */             }
/* FP:config.rs-2102 */             Some("short") => {
/* FP:config.rs-2103 */                 ErrorOutputType::HumanReadable { kind: HumanReadableErrorType::Short, color_config }
/* FP:config.rs-2104 */             }
/* FP:config.rs-2105 */             Some("human-unicode") => ErrorOutputType::HumanReadable {
/* FP:config.rs-2106 */                 kind: HumanReadableErrorType::Unicode,
/* FP:config.rs-2107 */                 color_config,
/* FP:config.rs-2108 */             },
/* FP:config.rs-2109 */             Some(arg) => {
/* FP:config.rs-2110 */                 early_dcx.set_error_format(ErrorOutputType::HumanReadable { color_config, .. });
/* FP:config.rs-2111 */                 early_dcx.early_fatal(format!(
/* FP:config.rs-2112 */                     "argument for `--error-format` must be `human`, `human-annotate-rs`, \
/* FP:config.rs-2113 */                     `human-unicode`, `json`, `pretty-json` or `short` (instead was `{arg}`)"
/* FP:config.rs-2114 */                 ))
/* FP:config.rs-2115 */             }
/* FP:config.rs-2116 */         }
/* FP:config.rs-2117 */     } else {
/* FP:config.rs-2118 */         ErrorOutputType::HumanReadable { color_config, .. }
/* FP:config.rs-2119 */     };
/* FP:config.rs-2120 */ 
/* FP:config.rs-2121 */     match error_format {
/* FP:config.rs-2122 */         ErrorOutputType::Json { .. } => {}
/* FP:config.rs-2123 */ 
/* FP:config.rs-2124 */         // Conservatively require that the `--json` argument is coupled with
/* FP:config.rs-2125 */         // `--error-format=json`. This means that `--json` is specified we
/* FP:config.rs-2126 */         // should actually be emitting JSON blobs.
/* FP:config.rs-2127 */         _ if !matches.opt_strs("json").is_empty() => {
/* FP:config.rs-2128 */             early_dcx.early_fatal("using `--json` requires also using `--error-format=json`");
/* FP:config.rs-2129 */         }
/* FP:config.rs-2130 */ 
/* FP:config.rs-2131 */         _ => {}
/* FP:config.rs-2132 */     }
/* FP:config.rs-2133 */ 
/* FP:config.rs-2134 */     error_format
/* FP:config.rs-2135 */ }
/* FP:config.rs-2136 */ 
/* FP:config.rs-2137 */ pub fn parse_crate_edition(early_dcx: &EarlyDiagCtxt, matches: &getopts::Matches) -> Edition {
/* FP:config.rs-2138 */     let edition = match matches.opt_str("edition") {
/* FP:config.rs-2139 */         Some(arg) => Edition::from_str(&arg).unwrap_or_else(|_| {
/* FP:config.rs-2140 */             early_dcx.early_fatal(format!(
/* FP:config.rs-2141 */                 "argument for `--edition` must be one of: \
/* FP:config.rs-2142 */                      {EDITION_NAME_LIST}. (instead was `{arg}`)"
/* FP:config.rs-2143 */             ))
/* FP:config.rs-2144 */         }),
/* FP:config.rs-2145 */         None => DEFAULT_EDITION,
/* FP:config.rs-2146 */     };
/* FP:config.rs-2147 */ 
/* FP:config.rs-2148 */     if !edition.is_stable() && !nightly_options::is_unstable_enabled(matches) {
/* FP:config.rs-2149 */         let is_nightly = nightly_options::match_is_nightly_build(matches);
/* FP:config.rs-2150 */         let msg = if !is_nightly {
/* FP:config.rs-2151 */             format!(
/* FP:config.rs-2152 */                 "the crate requires edition {edition}, but the latest edition supported by this Rust version is {LATEST_STABLE_EDITION}"
/* FP:config.rs-2153 */             )
/* FP:config.rs-2154 */         } else {
/* FP:config.rs-2155 */             format!("edition {edition} is unstable and only available with -Z unstable-options")
/* FP:config.rs-2156 */         };
/* FP:config.rs-2157 */         early_dcx.early_fatal(msg)
/* FP:config.rs-2158 */     }
/* FP:config.rs-2159 */ 
/* FP:config.rs-2160 */     edition
/* FP:config.rs-2161 */ }
/* FP:config.rs-2162 */ 
/* FP:config.rs-2163 */ fn check_error_format_stability(
/* FP:config.rs-2164 */     early_dcx: &EarlyDiagCtxt,
/* FP:config.rs-2165 */     unstable_opts: &UnstableOptions,
/* FP:config.rs-2166 */     format: ErrorOutputType,
/* FP:config.rs-2167 */ ) {
/* FP:config.rs-2168 */     if unstable_opts.unstable_options {
/* FP:config.rs-2169 */         return;
/* FP:config.rs-2170 */     }
/* FP:config.rs-2171 */     let format = match format {
/* FP:config.rs-2172 */         ErrorOutputType::Json { pretty: true, .. } => "pretty-json",
/* FP:config.rs-2173 */         ErrorOutputType::HumanReadable { kind, .. } => match kind {
/* FP:config.rs-2174 */             HumanReadableErrorType::AnnotateSnippet => "human-annotate-rs",
/* FP:config.rs-2175 */             HumanReadableErrorType::Unicode => "human-unicode",
/* FP:config.rs-2176 */             _ => return,
/* FP:config.rs-2177 */         },
/* FP:config.rs-2178 */         _ => return,
/* FP:config.rs-2179 */     };
/* FP:config.rs-2180 */     early_dcx.early_fatal(format!("`--error-format={format}` is unstable"))
/* FP:config.rs-2181 */ }
/* FP:config.rs-2182 */ 
/* FP:config.rs-2183 */ fn parse_output_types(
/* FP:config.rs-2184 */     early_dcx: &EarlyDiagCtxt,
/* FP:config.rs-2185 */     unstable_opts: &UnstableOptions,
/* FP:config.rs-2186 */     matches: &getopts::Matches,
/* FP:config.rs-2187 */ ) -> OutputTypes {
/* FP:config.rs-2188 */     let mut output_types = BTreeMap::new();
/* FP:config.rs-2189 */     if !unstable_opts.parse_crate_root_only {
/* FP:config.rs-2190 */         for list in matches.opt_strs("emit") {
/* FP:config.rs-2191 */             for output_type in list.split(',') {
/* FP:config.rs-2192 */                 let (shorthand, path) = split_out_file_name(output_type);
/* FP:config.rs-2193 */                 let output_type = OutputType::from_shorthand(shorthand).unwrap_or_else(|| {
/* FP:config.rs-2194 */                     early_dcx.early_fatal(format!(
/* FP:config.rs-2195 */                         "unknown emission type: `{shorthand}` - expected one of: {display}",
/* FP:config.rs-2196 */                         display = OutputType::shorthands_display(),
/* FP:config.rs-2197 */                     ))
/* FP:config.rs-2198 */                 });
/* FP:config.rs-2199 */                 if output_type == OutputType::ThinLinkBitcode && !unstable_opts.unstable_options {
/* FP:config.rs-2200 */                     early_dcx.early_fatal(format!(
/* FP:config.rs-2201 */                         "{} requested but -Zunstable-options not specified",
/* FP:config.rs-2202 */                         OutputType::ThinLinkBitcode.shorthand()
/* FP:config.rs-2203 */                     ));
/* FP:config.rs-2204 */                 }
/* FP:config.rs-2205 */                 output_types.insert(output_type, path);
/* FP:config.rs-2206 */             }
/* FP:config.rs-2207 */         }
/* FP:config.rs-2208 */     };
/* FP:config.rs-2209 */     if output_types.is_empty() {
/* FP:config.rs-2210 */         output_types.insert(OutputType::Exe, None);
/* FP:config.rs-2211 */     }
/* FP:config.rs-2212 */     OutputTypes(output_types)
/* FP:config.rs-2213 */ }
/* FP:config.rs-2214 */ 
/* FP:config.rs-2215 */ fn split_out_file_name(arg: &str) -> (&str, Option<OutFileName>) {
/* FP:config.rs-2216 */     match arg.split_once('=') {
/* FP:config.rs-2217 */         None => (arg, None),
/* FP:config.rs-2218 */         Some((kind, "-")) => (kind, Some(OutFileName::Stdout)),
/* FP:config.rs-2219 */         Some((kind, path)) => (kind, Some(OutFileName::Real(PathBuf::from(path)))),
/* FP:config.rs-2220 */     }
/* FP:config.rs-2221 */ }
/* FP:config.rs-2222 */ 
/* FP:config.rs-2223 */ fn should_override_cgus_and_disable_thinlto(
/* FP:config.rs-2224 */     early_dcx: &EarlyDiagCtxt,
/* FP:config.rs-2225 */     output_types: &OutputTypes,
/* FP:config.rs-2226 */     matches: &getopts::Matches,
/* FP:config.rs-2227 */     mut codegen_units: Option<usize>,
/* FP:config.rs-2228 */ ) -> (bool, Option<usize>) {
/* FP:config.rs-2229 */     let mut disable_local_thinlto = false;
/* FP:config.rs-2230 */     // Issue #30063: if user requests LLVM-related output to one
/* FP:config.rs-2231 */     // particular path, disable codegen-units.
/* FP:config.rs-2232 */     let incompatible: Vec<_> = output_types
/* FP:config.rs-2233 */         .0
/* FP:config.rs-2234 */         .iter()
/* FP:config.rs-2235 */         .map(|ot_path| ot_path.0)
/* FP:config.rs-2236 */         .filter(|ot| !ot.is_compatible_with_codegen_units_and_single_output_file())
/* FP:config.rs-2237 */         .map(|ot| ot.shorthand())
/* FP:config.rs-2238 */         .collect();
/* FP:config.rs-2239 */     if !incompatible.is_empty() {
/* FP:config.rs-2240 */         match codegen_units {
/* FP:config.rs-2241 */             Some(n) if n > 1 => {
/* FP:config.rs-2242 */                 if matches.opt_present("o") {
/* FP:config.rs-2243 */                     for ot in &incompatible {
/* FP:config.rs-2244 */                         early_dcx.early_warn(format!(
/* FP:config.rs-2245 */                             "`--emit={ot}` with `-o` incompatible with \
/* FP:config.rs-2246 */                                  `-C codegen-units=N` for N > 1",
/* FP:config.rs-2247 */                         ));
/* FP:config.rs-2248 */                     }
/* FP:config.rs-2249 */                     early_dcx.early_warn("resetting to default -C codegen-units=1");
/* FP:config.rs-2250 */                     codegen_units = Some(1);
/* FP:config.rs-2251 */                     disable_local_thinlto = true;
/* FP:config.rs-2252 */                 }
/* FP:config.rs-2253 */             }
/* FP:config.rs-2254 */             _ => {
/* FP:config.rs-2255 */                 codegen_units = Some(1);
/* FP:config.rs-2256 */                 disable_local_thinlto = true;
/* FP:config.rs-2257 */             }
/* FP:config.rs-2258 */         }
/* FP:config.rs-2259 */     }
/* FP:config.rs-2260 */ 
/* FP:config.rs-2261 */     if codegen_units == Some(0) {
/* FP:config.rs-2262 */         early_dcx.early_fatal("value for codegen units must be a positive non-zero integer");
/* FP:config.rs-2263 */     }
/* FP:config.rs-2264 */ 
/* FP:config.rs-2265 */     (disable_local_thinlto, codegen_units)
/* FP:config.rs-2266 */ }
/* FP:config.rs-2267 */ 
/* FP:config.rs-2268 */ fn collect_print_requests(
/* FP:config.rs-2269 */     early_dcx: &EarlyDiagCtxt,
/* FP:config.rs-2270 */     cg: &mut CodegenOptions,
/* FP:config.rs-2271 */     unstable_opts: &UnstableOptions,
/* FP:config.rs-2272 */     matches: &getopts::Matches,
/* FP:config.rs-2273 */ ) -> Vec<PrintRequest> {
/* FP:config.rs-2274 */     let mut prints = Vec::<PrintRequest>::new();
/* FP:config.rs-2275 */     if cg.target_cpu.as_deref() == Some("help") {
/* FP:config.rs-2276 */         prints.push(PrintRequest { kind: PrintKind::TargetCPUs, out: OutFileName::Stdout });
/* FP:config.rs-2277 */         cg.target_cpu = None;
/* FP:config.rs-2278 */     };
/* FP:config.rs-2279 */     if cg.target_feature == "help" {
/* FP:config.rs-2280 */         prints.push(PrintRequest { kind: PrintKind::TargetFeatures, out: OutFileName::Stdout });
/* FP:config.rs-2281 */         cg.target_feature = String::new();
/* FP:config.rs-2282 */     }
/* FP:config.rs-2283 */ 
/* FP:config.rs-2284 */     // We disallow reusing the same path in multiple prints, such as `--print
/* FP:config.rs-2285 */     // cfg=output.txt --print link-args=output.txt`, because outputs are printed
/* FP:config.rs-2286 */     // by disparate pieces of the compiler, and keeping track of which files
/* FP:config.rs-2287 */     // need to be overwritten vs appended to is annoying.
/* FP:config.rs-2288 */     let mut printed_paths = FxHashSet::default();
/* FP:config.rs-2289 */ 
/* FP:config.rs-2290 */     prints.extend(matches.opt_strs("print").into_iter().map(|req| {
/* FP:config.rs-2291 */         let (req, out) = split_out_file_name(&req);
/* FP:config.rs-2292 */ 
/* FP:config.rs-2293 */         let kind = if let Some((print_name, print_kind)) =
/* FP:config.rs-2294 */             PRINT_KINDS.iter().find(|&&(name, _)| name == req)
/* FP:config.rs-2295 */         {
/* FP:config.rs-2296 */             check_print_request_stability(early_dcx, unstable_opts, (print_name, *print_kind));
/* FP:config.rs-2297 */             *print_kind
/* FP:config.rs-2298 */         } else {
/* FP:config.rs-2299 */             let is_nightly = nightly_options::match_is_nightly_build(matches);
/* FP:config.rs-2300 */             emit_unknown_print_request_help(early_dcx, req, is_nightly)
/* FP:config.rs-2301 */         };
/* FP:config.rs-2302 */ 
/* FP:config.rs-2303 */         let out = out.unwrap_or(OutFileName::Stdout);
/* FP:config.rs-2304 */         if let OutFileName::Real(path) = &out {
/* FP:config.rs-2305 */             if !printed_paths.insert(path.clone()) {
/* FP:config.rs-2306 */                 early_dcx.early_fatal(format!(
/* FP:config.rs-2307 */                     "cannot print multiple outputs to the same path: {}",
/* FP:config.rs-2308 */                     path.display(),
/* FP:config.rs-2309 */                 ));
/* FP:config.rs-2310 */             }
/* FP:config.rs-2311 */         }
/* FP:config.rs-2312 */ 
/* FP:config.rs-2313 */         PrintRequest { kind, out }
/* FP:config.rs-2314 */     }));
/* FP:config.rs-2315 */ 
/* FP:config.rs-2316 */     prints
/* FP:config.rs-2317 */ }
/* FP:config.rs-2318 */ 
/* FP:config.rs-2319 */ fn check_print_request_stability(
/* FP:config.rs-2320 */     early_dcx: &EarlyDiagCtxt,
/* FP:config.rs-2321 */     unstable_opts: &UnstableOptions,
/* FP:config.rs-2322 */     (print_name, print_kind): (&str, PrintKind),
/* FP:config.rs-2323 */ ) {
/* FP:config.rs-2324 */     if !is_print_request_stable(print_kind) && !unstable_opts.unstable_options {
/* FP:config.rs-2325 */         early_dcx.early_fatal(format!(
/* FP:config.rs-2326 */             "the `-Z unstable-options` flag must also be passed to enable the `{print_name}` \
/* FP:config.rs-2327 */                 print option"
/* FP:config.rs-2328 */         ));
/* FP:config.rs-2329 */     }
/* FP:config.rs-2330 */ }
/* FP:config.rs-2331 */ 
/* FP:config.rs-2332 */ fn is_print_request_stable(print_kind: PrintKind) -> bool {
/* FP:config.rs-2333 */     match print_kind {
/* FP:config.rs-2334 */         PrintKind::AllTargetSpecsJson
/* FP:config.rs-2335 */         | PrintKind::CheckCfg
/* FP:config.rs-2336 */         | PrintKind::CrateRootLintLevels
/* FP:config.rs-2337 */         | PrintKind::SupportedCrateTypes
/* FP:config.rs-2338 */         | PrintKind::TargetSpecJson
/* FP:config.rs-2339 */         | PrintKind::TargetSpecJsonSchema => false,
/* FP:config.rs-2340 */         _ => true,
/* FP:config.rs-2341 */     }
/* FP:config.rs-2342 */ }
/* FP:config.rs-2343 */ 
/* FP:config.rs-2344 */ fn emit_unknown_print_request_help(early_dcx: &EarlyDiagCtxt, req: &str, is_nightly: bool) -> ! {
/* FP:config.rs-2345 */     let prints = PRINT_KINDS
/* FP:config.rs-2346 */         .iter()
/* FP:config.rs-2347 */         .filter_map(|(name, kind)| {
/* FP:config.rs-2348 */             // If we're not on nightly, we don't want to print unstable options
/* FP:config.rs-2349 */             if !is_nightly && !is_print_request_stable(*kind) {
/* FP:config.rs-2350 */                 None
/* FP:config.rs-2351 */             } else {
/* FP:config.rs-2352 */                 Some(format!("`{name}`"))
/* FP:config.rs-2353 */             }
/* FP:config.rs-2354 */         })
/* FP:config.rs-2355 */         .collect::<Vec<_>>();
/* FP:config.rs-2356 */     let prints = prints.join(", ");
/* FP:config.rs-2357 */ 
/* FP:config.rs-2358 */     let mut diag = early_dcx.early_struct_fatal(format!("unknown print request: `{req}`"));
/* FP:config.rs-2359 */     #[allow(rustc::diagnostic_outside_of_impl)]
/* FP:config.rs-2360 */     diag.help(format!("valid print requests are: {prints}"));
/* FP:config.rs-2361 */ 
/* FP:config.rs-2362 */     if req == "lints" {
/* FP:config.rs-2363 */         diag.help(format!("use `-Whelp` to print a list of lints"));
/* FP:config.rs-2364 */     }
/* FP:config.rs-2365 */ 
/* FP:config.rs-2366 */     diag.help(format!("for more information, see the rustc book: https://doc.rust-lang.org/rustc/command-line-arguments.html#--print-print-compiler-information"));
/* FP:config.rs-2367 */     diag.emit()
/* FP:config.rs-2368 */ }
/* FP:config.rs-2369 */ 
/* FP:config.rs-2370 */ pub fn parse_target_triple(early_dcx: &EarlyDiagCtxt, matches: &getopts::Matches) -> TargetTuple {
/* FP:config.rs-2371 */     match matches.opt_str("target") {
/* FP:config.rs-2372 */         Some(target) if target.ends_with(".json") => {
/* FP:config.rs-2373 */             let path = Path::new(&target);
/* FP:config.rs-2374 */             TargetTuple::from_path(path).unwrap_or_else(|_| {
/* FP:config.rs-2375 */                 early_dcx.early_fatal(format!("target file {path:?} does not exist"))
/* FP:config.rs-2376 */             })
/* FP:config.rs-2377 */         }
/* FP:config.rs-2378 */         Some(target) => TargetTuple::TargetTuple(target),
/* FP:config.rs-2379 */         _ => TargetTuple::from_tuple(host_tuple()),
/* FP:config.rs-2380 */     }
/* FP:config.rs-2381 */ }
/* FP:config.rs-2382 */ 
/* FP:config.rs-2383 */ fn parse_opt_level(
/* FP:config.rs-2384 */     early_dcx: &EarlyDiagCtxt,
/* FP:config.rs-2385 */     matches: &getopts::Matches,
/* FP:config.rs-2386 */     cg: &CodegenOptions,
/* FP:config.rs-2387 */ ) -> OptLevel {
/* FP:config.rs-2388 */     // The `-O` and `-C opt-level` flags specify the same setting, so we want to be able
/* FP:config.rs-2389 */     // to use them interchangeably. However, because they're technically different flags,
/* FP:config.rs-2390 */     // we need to work out manually which should take precedence if both are supplied (i.e.
/* FP:config.rs-2391 */     // the rightmost flag). We do this by finding the (rightmost) position of both flags and
/* FP:config.rs-2392 */     // comparing them. Note that if a flag is not found, its position will be `None`, which
/* FP:config.rs-2393 */     // always compared less than `Some(_)`.
/* FP:config.rs-2394 */     let max_o = matches.opt_positions("O").into_iter().max();
/* FP:config.rs-2395 */     let max_c = matches
/* FP:config.rs-2396 */         .opt_strs_pos("C")
/* FP:config.rs-2397 */         .into_iter()
/* FP:config.rs-2398 */         .flat_map(|(i, s)| {
/* FP:config.rs-2399 */             // NB: This can match a string without `=`.
/* FP:config.rs-2400 */             if let Some("opt-level") = s.split('=').next() { Some(i) } else { None }
/* FP:config.rs-2401 */         })
/* FP:config.rs-2402 */         .max();
/* FP:config.rs-2403 */     if max_o > max_c {
/* FP:config.rs-2404 */         OptLevel::Aggressive
/* FP:config.rs-2405 */     } else {
/* FP:config.rs-2406 */         match cg.opt_level.as_ref() {
/* FP:config.rs-2407 */             "0" => OptLevel::No,
/* FP:config.rs-2408 */             "1" => OptLevel::Less,
/* FP:config.rs-2409 */             "2" => OptLevel::More,
/* FP:config.rs-2410 */             "3" => OptLevel::Aggressive,
/* FP:config.rs-2411 */             "s" => OptLevel::Size,
/* FP:config.rs-2412 */             "z" => OptLevel::SizeMin,
/* FP:config.rs-2413 */             arg => {
/* FP:config.rs-2414 */                 early_dcx.early_fatal(format!(
/* FP:config.rs-2415 */                     "optimization level needs to be \
/* FP:config.rs-2416 */                             between 0-3, s or z (instead was `{arg}`)"
/* FP:config.rs-2417 */                 ));
/* FP:config.rs-2418 */             }
/* FP:config.rs-2419 */         }
/* FP:config.rs-2420 */     }
/* FP:config.rs-2421 */ }
/* FP:config.rs-2422 */ 
/* FP:config.rs-2423 */ fn select_debuginfo(matches: &getopts::Matches, cg: &CodegenOptions) -> DebugInfo {
/* FP:config.rs-2424 */     let max_g = matches.opt_positions("g").into_iter().max();
/* FP:config.rs-2425 */     let max_c = matches
/* FP:config.rs-2426 */         .opt_strs_pos("C")
/* FP:config.rs-2427 */         .into_iter()
/* FP:config.rs-2428 */         .flat_map(|(i, s)| {
/* FP:config.rs-2429 */             // NB: This can match a string without `=`.
/* FP:config.rs-2430 */             if let Some("debuginfo") = s.split('=').next() { Some(i) } else { None }
/* FP:config.rs-2431 */         })
/* FP:config.rs-2432 */         .max();
/* FP:config.rs-2433 */     if max_g > max_c { DebugInfo::Full } else { cg.debuginfo }
/* FP:config.rs-2434 */ }
/* FP:config.rs-2435 */ 
/* FP:config.rs-2436 */ fn parse_assert_incr_state(
/* FP:config.rs-2437 */     early_dcx: &EarlyDiagCtxt,
/* FP:config.rs-2438 */     opt_assertion: &Option<String>,
/* FP:config.rs-2439 */ ) -> Option<IncrementalStateAssertion> {
/* FP:config.rs-2440 */     match opt_assertion {
/* FP:config.rs-2441 */         Some(s) if s.as_str() == "loaded" => Some(IncrementalStateAssertion::Loaded),
/* FP:config.rs-2442 */         Some(s) if s.as_str() == "not-loaded" => Some(IncrementalStateAssertion::NotLoaded),
/* FP:config.rs-2443 */         Some(s) => {
/* FP:config.rs-2444 */             early_dcx.early_fatal(format!("unexpected incremental state assertion value: {s}"))
/* FP:config.rs-2445 */         }
/* FP:config.rs-2446 */         None => None,
/* FP:config.rs-2447 */     }
/* FP:config.rs-2448 */ }
/* FP:config.rs-2449 */ 
/* FP:config.rs-2450 */ pub fn parse_externs(
/* FP:config.rs-2451 */     early_dcx: &EarlyDiagCtxt,
/* FP:config.rs-2452 */     matches: &getopts::Matches,
/* FP:config.rs-2453 */     unstable_opts: &UnstableOptions,
/* FP:config.rs-2454 */ ) -> Externs {
/* FP:config.rs-2455 */     let is_unstable_enabled = unstable_opts.unstable_options;
/* FP:config.rs-2456 */     let mut externs: BTreeMap<String, ExternEntry> = BTreeMap::new();
/* FP:config.rs-2457 */     for arg in matches.opt_strs("extern") {
/* FP:config.rs-2458 */         let ExternOpt { crate_name: name, path, options } =
/* FP:config.rs-2459 */             split_extern_opt(early_dcx, unstable_opts, &arg).unwrap_or_else(|e| e.emit());
/* FP:config.rs-2460 */ 
/* FP:config.rs-2461 */         let entry = externs.entry(name.to_owned());
/* FP:config.rs-2462 */ 
/* FP:config.rs-2463 */         use std::collections::btree_map::Entry;
/* FP:config.rs-2464 */ 
/* FP:config.rs-2465 */         let entry = if let Some(path) = path {
/* FP:config.rs-2466 */             // --extern prelude_name=some_file.rlib
/* FP:config.rs-2467 */             let path = CanonicalizedPath::new(path);
/* FP:config.rs-2468 */             match entry {
/* FP:config.rs-2469 */                 Entry::Vacant(vacant) => {
/* FP:config.rs-2470 */                     let files = BTreeSet::from_iter(iter::once(path));
/* FP:config.rs-2471 */                     vacant.insert(ExternEntry::new(ExternLocation::ExactPaths(files)))
/* FP:config.rs-2472 */                 }
/* FP:config.rs-2473 */                 Entry::Occupied(occupied) => {
/* FP:config.rs-2474 */                     let ext_ent = occupied.into_mut();
/* FP:config.rs-2475 */                     match ext_ent {
/* FP:config.rs-2476 */                         ExternEntry { location: ExternLocation::ExactPaths(files), .. } => {
/* FP:config.rs-2477 */                             files.insert(path);
/* FP:config.rs-2478 */                         }
/* FP:config.rs-2479 */                         ExternEntry {
/* FP:config.rs-2480 */                             location: location @ ExternLocation::FoundInLibrarySearchDirectories,
/* FP:config.rs-2481 */                             ..
/* FP:config.rs-2482 */                         } => {
/* FP:config.rs-2483 */                             // Exact paths take precedence over search directories.
/* FP:config.rs-2484 */                             let files = BTreeSet::from_iter(iter::once(path));
/* FP:config.rs-2485 */                             *location = ExternLocation::ExactPaths(files);
/* FP:config.rs-2486 */                         }
/* FP:config.rs-2487 */                     }
/* FP:config.rs-2488 */                     ext_ent
/* FP:config.rs-2489 */                 }
/* FP:config.rs-2490 */             }
/* FP:config.rs-2491 */         } else {
/* FP:config.rs-2492 */             // --extern prelude_name
/* FP:config.rs-2493 */             match entry {
/* FP:config.rs-2494 */                 Entry::Vacant(vacant) => {
/* FP:config.rs-2495 */                     vacant.insert(ExternEntry::new(ExternLocation::FoundInLibrarySearchDirectories))
/* FP:config.rs-2496 */                 }
/* FP:config.rs-2497 */                 Entry::Occupied(occupied) => {
/* FP:config.rs-2498 */                     // Ignore if already specified.
/* FP:config.rs-2499 */                     occupied.into_mut()
/* FP:config.rs-2500 */                 }
/* FP:config.rs-2501 */             }
/* FP:config.rs-2502 */         };
/* FP:config.rs-2503 */ 
/* FP:config.rs-2504 */         let mut is_private_dep = false;
/* FP:config.rs-2505 */         let mut add_prelude = true;
/* FP:config.rs-2506 */         let mut nounused_dep = false;
/* FP:config.rs-2507 */         let mut force = false;
/* FP:config.rs-2508 */         if let Some(opts) = options {
/* FP:config.rs-2509 */             if !is_unstable_enabled {
/* FP:config.rs-2510 */                 early_dcx.early_fatal(
/* FP:config.rs-2511 */                     "the `-Z unstable-options` flag must also be passed to \
/* FP:config.rs-2512 */                      enable `--extern` options",
/* FP:config.rs-2513 */                 );
/* FP:config.rs-2514 */             }
/* FP:config.rs-2515 */             for opt in opts.split(',') {
/* FP:config.rs-2516 */                 match opt {
/* FP:config.rs-2517 */                     "priv" => is_private_dep = true,
/* FP:config.rs-2518 */                     "noprelude" => {
/* FP:config.rs-2519 */                         if let ExternLocation::ExactPaths(_) = &entry.location {
/* FP:config.rs-2520 */                             add_prelude = false;
/* FP:config.rs-2521 */                         } else {
/* FP:config.rs-2522 */                             early_dcx.early_fatal(
/* FP:config.rs-2523 */                                 "the `noprelude` --extern option requires a file path",
/* FP:config.rs-2524 */                             );
/* FP:config.rs-2525 */                         }
/* FP:config.rs-2526 */                     }
/* FP:config.rs-2527 */                     "nounused" => nounused_dep = true,
/* FP:config.rs-2528 */                     "force" => force = true,
/* FP:config.rs-2529 */                     _ => early_dcx.early_fatal(format!("unknown --extern option `{opt}`")),
/* FP:config.rs-2530 */                 }
/* FP:config.rs-2531 */             }
/* FP:config.rs-2532 */         }
/* FP:config.rs-2533 */ 
/* FP:config.rs-2534 */         // Crates start out being not private, and go to being private `priv`
/* FP:config.rs-2535 */         // is specified.
/* FP:config.rs-2536 */         entry.is_private_dep |= is_private_dep;
/* FP:config.rs-2537 */         // likewise `nounused`
/* FP:config.rs-2538 */         entry.nounused_dep |= nounused_dep;
/* FP:config.rs-2539 */         // and `force`
/* FP:config.rs-2540 */         entry.force |= force;
/* FP:config.rs-2541 */         // If any flag is missing `noprelude`, then add to the prelude.
/* FP:config.rs-2542 */         entry.add_prelude |= add_prelude;
/* FP:config.rs-2543 */     }
/* FP:config.rs-2544 */     Externs(externs)
/* FP:config.rs-2545 */ }
/* FP:config.rs-2546 */ 
/* FP:config.rs-2547 */ fn parse_remap_path_prefix(
/* FP:config.rs-2548 */     early_dcx: &EarlyDiagCtxt,
/* FP:config.rs-2549 */     matches: &getopts::Matches,
/* FP:config.rs-2550 */     unstable_opts: &UnstableOptions,
/* FP:config.rs-2551 */ ) -> Vec<(PathBuf, PathBuf)> {
/* FP:config.rs-2552 */     let mut mapping: Vec<(PathBuf, PathBuf)> = matches
/* FP:config.rs-2553 */         .opt_strs("remap-path-prefix")
/* FP:config.rs-2554 */         .into_iter()
/* FP:config.rs-2555 */         .map(|remap| match remap.rsplit_once('=') {
/* FP:config.rs-2556 */             None => {
/* FP:config.rs-2557 */                 early_dcx.early_fatal("--remap-path-prefix must contain '=' between FROM and TO")
/* FP:config.rs-2558 */             }
/* FP:config.rs-2559 */             Some((from, to)) => (PathBuf::from(from), PathBuf::from(to)),
/* FP:config.rs-2560 */         })
/* FP:config.rs-2561 */         .collect();
/* FP:config.rs-2562 */     match &unstable_opts.remap_cwd_prefix {
/* FP:config.rs-2563 */         Some(to) => match std::env::current_dir() {
/* FP:config.rs-2564 */             Ok(cwd) => mapping.push((cwd, to.clone())),
/* FP:config.rs-2565 */             Err(_) => (),
/* FP:config.rs-2566 */         },
/* FP:config.rs-2567 */         None => (),
/* FP:config.rs-2568 */     };
/* FP:config.rs-2569 */     mapping
/* FP:config.rs-2570 */ }
/* FP:config.rs-2571 */ 
/* FP:config.rs-2572 */ fn parse_logical_env(
/* FP:config.rs-2573 */     early_dcx: &EarlyDiagCtxt,
/* FP:config.rs-2574 */     matches: &getopts::Matches,
/* FP:config.rs-2575 */ ) -> FxIndexMap<String, String> {
/* FP:config.rs-2576 */     let mut vars = FxIndexMap::default();
/* FP:config.rs-2577 */ 
/* FP:config.rs-2578 */     for arg in matches.opt_strs("env-set") {
/* FP:config.rs-2579 */         if let Some((name, val)) = arg.split_once('=') {
/* FP:config.rs-2580 */             vars.insert(name.to_string(), val.to_string());
/* FP:config.rs-2581 */         } else {
/* FP:config.rs-2582 */             early_dcx.early_fatal(format!("`--env-set`: specify value for variable `{arg}`"));
/* FP:config.rs-2583 */         }
/* FP:config.rs-2584 */     }
/* FP:config.rs-2585 */ 
/* FP:config.rs-2586 */     vars
/* FP:config.rs-2587 */ }
/* FP:config.rs-2588 */ 
/* FP:config.rs-2589 */ // JUSTIFICATION: before wrapper fn is available
/* FP:config.rs-2590 */ #[allow(rustc::bad_opt_access)]
/* FP:config.rs-2591 */ pub fn build_session_options(early_dcx: &mut EarlyDiagCtxt, matches: &getopts::Matches) -> Options {
/* FP:config.rs-2592 */     let color = parse_color(early_dcx, matches);
/* FP:config.rs-2593 */ 
/* FP:config.rs-2594 */     let edition = parse_crate_edition(early_dcx, matches);
/* FP:config.rs-2595 */ 
/* FP:config.rs-2596 */     let JsonConfig {
/* FP:config.rs-2597 */         json_rendered,
/* FP:config.rs-2598 */         json_color,
/* FP:config.rs-2599 */         json_artifact_notifications,
/* FP:config.rs-2600 */         json_timings,
/* FP:config.rs-2601 */         json_unused_externs,
/* FP:config.rs-2602 */         json_future_incompat,
/* FP:config.rs-2603 */     } = parse_json(early_dcx, matches);
/* FP:config.rs-2604 */ 
/* FP:config.rs-2605 */     let error_format = parse_error_format(early_dcx, matches, color, json_color, json_rendered);
/* FP:config.rs-2606 */ 
/* FP:config.rs-2607 */     early_dcx.set_error_format(error_format);
/* FP:config.rs-2608 */ 
/* FP:config.rs-2609 */     let diagnostic_width = matches.opt_get("diagnostic-width").unwrap_or_else(|_| {
/* FP:config.rs-2610 */         early_dcx.early_fatal("`--diagnostic-width` must be an positive integer");
/* FP:config.rs-2611 */     });
/* FP:config.rs-2612 */ 
/* FP:config.rs-2613 */     let unparsed_crate_types = matches.opt_strs("crate-type");
/* FP:config.rs-2614 */     let crate_types = parse_crate_types_from_list(unparsed_crate_types)
/* FP:config.rs-2615 */         .unwrap_or_else(|e| early_dcx.early_fatal(e));
/* FP:config.rs-2616 */ 
/* FP:config.rs-2617 */     let mut target_modifiers = BTreeMap::<OptionsTargetModifiers, String>::new();
/* FP:config.rs-2618 */ 
/* FP:config.rs-2619 */     let mut unstable_opts = UnstableOptions::build(early_dcx, matches, &mut target_modifiers);
/* FP:config.rs-2620 */     let (lint_opts, describe_lints, lint_cap) = get_cmd_lint_options(early_dcx, matches);
/* FP:config.rs-2621 */ 
/* FP:config.rs-2622 */     if !unstable_opts.unstable_options && json_timings {
/* FP:config.rs-2623 */         early_dcx.early_fatal("--json=timings is unstable and requires using `-Zunstable-options`");
/* FP:config.rs-2624 */     }
/* FP:config.rs-2625 */ 
/* FP:config.rs-2626 */     check_error_format_stability(early_dcx, &unstable_opts, error_format);
/* FP:config.rs-2627 */ 
/* FP:config.rs-2628 */     let output_types = parse_output_types(early_dcx, &unstable_opts, matches);
/* FP:config.rs-2629 */ 
/* FP:config.rs-2630 */     let mut cg = CodegenOptions::build(early_dcx, matches, &mut target_modifiers);
/* FP:config.rs-2631 */     let (disable_local_thinlto, codegen_units) = should_override_cgus_and_disable_thinlto(
/* FP:config.rs-2632 */         early_dcx,
/* FP:config.rs-2633 */         &output_types,
/* FP:config.rs-2634 */         matches,
/* FP:config.rs-2635 */         cg.codegen_units,
/* FP:config.rs-2636 */     );
/* FP:config.rs-2637 */ 
/* FP:config.rs-2638 */     if unstable_opts.threads == 0 {
/* FP:config.rs-2639 */         early_dcx.early_fatal("value for threads must be a positive non-zero integer");
/* FP:config.rs-2640 */     }
/* FP:config.rs-2641 */ 
/* FP:config.rs-2642 */     if unstable_opts.threads == parse::MAX_THREADS_CAP {
/* FP:config.rs-2643 */         early_dcx.early_warn(format!("number of threads was capped at {}", parse::MAX_THREADS_CAP));
/* FP:config.rs-2644 */     }
/* FP:config.rs-2645 */ 
/* FP:config.rs-2646 */     let incremental = cg.incremental.as_ref().map(PathBuf::from);
/* FP:config.rs-2647 */ 
/* FP:config.rs-2648 */     let assert_incr_state = parse_assert_incr_state(early_dcx, &unstable_opts.assert_incr_state);
/* FP:config.rs-2649 */ 
/* FP:config.rs-2650 */     if cg.profile_generate.enabled() && cg.profile_use.is_some() {
/* FP:config.rs-2651 */         early_dcx.early_fatal("options `-C profile-generate` and `-C profile-use` are exclusive");
/* FP:config.rs-2652 */     }
/* FP:config.rs-2653 */ 
/* FP:config.rs-2654 */     if unstable_opts.profile_sample_use.is_some()
/* FP:config.rs-2655 */         && (cg.profile_generate.enabled() || cg.profile_use.is_some())
/* FP:config.rs-2656 */     {
/* FP:config.rs-2657 */         early_dcx.early_fatal(
/* FP:config.rs-2658 */             "option `-Z profile-sample-use` cannot be used with `-C profile-generate` or `-C profile-use`",
/* FP:config.rs-2659 */         );
/* FP:config.rs-2660 */     }
/* FP:config.rs-2661 */ 
/* FP:config.rs-2662 */     // Check for unstable values of `-C symbol-mangling-version`.
/* FP:config.rs-2663 */     // This is what prevents them from being used on stable compilers.
/* FP:config.rs-2664 */     match cg.symbol_mangling_version {
/* FP:config.rs-2665 */         // Stable values:
/* FP:config.rs-2666 */         None | Some(SymbolManglingVersion::V0) => {}
/* FP:config.rs-2667 */ 
/* FP:config.rs-2668 */         // Unstable values:
/* FP:config.rs-2669 */         Some(SymbolManglingVersion::Legacy) => {
/* FP:config.rs-2670 */             if !unstable_opts.unstable_options {
/* FP:config.rs-2671 */                 early_dcx.early_fatal(
/* FP:config.rs-2672 */                     "`-C symbol-mangling-version=legacy` requires `-Z unstable-options`",
/* FP:config.rs-2673 */                 );
/* FP:config.rs-2674 */             }
/* FP:config.rs-2675 */         }
/* FP:config.rs-2676 */         Some(SymbolManglingVersion::Hashed) => {
/* FP:config.rs-2677 */             if !unstable_opts.unstable_options {
/* FP:config.rs-2678 */                 early_dcx.early_fatal(
/* FP:config.rs-2679 */                     "`-C symbol-mangling-version=hashed` requires `-Z unstable-options`",
/* FP:config.rs-2680 */                 );
/* FP:config.rs-2681 */             }
/* FP:config.rs-2682 */         }
/* FP:config.rs-2683 */     }
/* FP:config.rs-2684 */ 
/* FP:config.rs-2685 */     if cg.instrument_coverage != InstrumentCoverage::No {
/* FP:config.rs-2686 */         if cg.profile_generate.enabled() || cg.profile_use.is_some() {
/* FP:config.rs-2687 */             early_dcx.early_fatal(
/* FP:config.rs-2688 */                 "option `-C instrument-coverage` is not compatible with either `-C profile-use` \
/* FP:config.rs-2689 */                 or `-C profile-generate`",
/* FP:config.rs-2690 */             );
/* FP:config.rs-2691 */         }
/* FP:config.rs-2692 */ 
/* FP:config.rs-2693 */         // `-C instrument-coverage` implies `-C symbol-mangling-version=v0` - to ensure consistent
/* FP:config.rs-2694 */         // and reversible name mangling. Note, LLVM coverage tools can analyze coverage over
/* FP:config.rs-2695 */         // multiple runs, including some changes to source code; so mangled names must be consistent
/* FP:config.rs-2696 */         // across compilations.
/* FP:config.rs-2697 */         match cg.symbol_mangling_version {
/* FP:config.rs-2698 */             None => cg.symbol_mangling_version = Some(SymbolManglingVersion::V0),
/* FP:config.rs-2699 */             Some(SymbolManglingVersion::Legacy) => {
/* FP:config.rs-2700 */                 early_dcx.early_warn(
/* FP:config.rs-2701 */                     "-C instrument-coverage requires symbol mangling version `v0`, \
/* FP:config.rs-2702 */                     but `-C symbol-mangling-version=legacy` was specified",
/* FP:config.rs-2703 */                 );
/* FP:config.rs-2704 */             }
/* FP:config.rs-2705 */             Some(SymbolManglingVersion::V0) => {}
/* FP:config.rs-2706 */             Some(SymbolManglingVersion::Hashed) => {
/* FP:config.rs-2707 */                 early_dcx.early_warn(
/* FP:config.rs-2708 */                     "-C instrument-coverage requires symbol mangling version `v0`, \
/* FP:config.rs-2709 */                     but `-C symbol-mangling-version=hashed` was specified",
/* FP:config.rs-2710 */                 );
/* FP:config.rs-2711 */             }
/* FP:config.rs-2712 */         }
/* FP:config.rs-2713 */     }
/* FP:config.rs-2714 */ 
/* FP:config.rs-2715 */     if let Ok(graphviz_font) = std::env::var("RUSTC_GRAPHVIZ_FONT") {
/* FP:config.rs-2716 */         // FIXME: this is only mutation of UnstableOptions here, move into
/* FP:config.rs-2717 */         // UnstableOptions::build?
/* FP:config.rs-2718 */         unstable_opts.graphviz_font = graphviz_font;
/* FP:config.rs-2719 */     }
/* FP:config.rs-2720 */ 
/* FP:config.rs-2721 */     if !cg.embed_bitcode {
/* FP:config.rs-2722 */         match cg.lto {
/* FP:config.rs-2723 */             LtoCli::No | LtoCli::Unspecified => {}
/* FP:config.rs-2724 */             LtoCli::Yes | LtoCli::NoParam | LtoCli::Thin | LtoCli::Fat => {
/* FP:config.rs-2725 */                 early_dcx.early_fatal("options `-C embed-bitcode=no` and `-C lto` are incompatible")
/* FP:config.rs-2726 */             }
/* FP:config.rs-2727 */         }
/* FP:config.rs-2728 */     }
/* FP:config.rs-2729 */ 
/* FP:config.rs-2730 */     let unstable_options_enabled = nightly_options::is_unstable_enabled(matches);
/* FP:config.rs-2731 */     if !unstable_options_enabled && cg.force_frame_pointers == FramePointer::NonLeaf {
/* FP:config.rs-2732 */         early_dcx.early_fatal(
/* FP:config.rs-2733 */             "`-Cforce-frame-pointers=non-leaf` or `always` also requires `-Zunstable-options` \
/* FP:config.rs-2734 */                 and a nightly compiler",
/* FP:config.rs-2735 */         )
/* FP:config.rs-2736 */     }
/* FP:config.rs-2737 */ 
/* FP:config.rs-2738 */     if !nightly_options::is_unstable_enabled(matches)
/* FP:config.rs-2739 */         && unstable_opts.offload.contains(&Offload::Enable)
/* FP:config.rs-2740 */     {
/* FP:config.rs-2741 */         early_dcx.early_fatal(
/* FP:config.rs-2742 */             "`-Zoffload=Enable` also requires `-Zunstable-options` \
/* FP:config.rs-2743 */                 and a nightly compiler",
/* FP:config.rs-2744 */         )
/* FP:config.rs-2745 */     }
/* FP:config.rs-2746 */ 
/* FP:config.rs-2747 */     let target_triple = parse_target_triple(early_dcx, matches);
/* FP:config.rs-2748 */ 
/* FP:config.rs-2749 */     // Ensure `-Z unstable-options` is required when using the unstable `-C link-self-contained` and
/* FP:config.rs-2750 */     // `-C linker-flavor` options.
/* FP:config.rs-2751 */     if !unstable_options_enabled {
/* FP:config.rs-2752 */         if let Err(error) = cg.link_self_contained.check_unstable_variants(&target_triple) {
/* FP:config.rs-2753 */             early_dcx.early_fatal(error);
/* FP:config.rs-2754 */         }
/* FP:config.rs-2755 */ 
/* FP:config.rs-2756 */         if let Some(flavor) = cg.linker_flavor {
/* FP:config.rs-2757 */             if flavor.is_unstable() {
/* FP:config.rs-2758 */                 early_dcx.early_fatal(format!(
/* FP:config.rs-2759 */                     "the linker flavor `{}` is unstable, the `-Z unstable-options` \
/* FP:config.rs-2760 */                         flag must also be passed to use the unstable values",
/* FP:config.rs-2761 */                     flavor.desc()
/* FP:config.rs-2762 */                 ));
/* FP:config.rs-2763 */             }
/* FP:config.rs-2764 */         }
/* FP:config.rs-2765 */     }
/* FP:config.rs-2766 */ 
/* FP:config.rs-2767 */     // Check `-C link-self-contained` for consistency: individual components cannot be both enabled
/* FP:config.rs-2768 */     // and disabled at the same time.
/* FP:config.rs-2769 */     if let Some(erroneous_components) = cg.link_self_contained.check_consistency() {
/* FP:config.rs-2770 */         let names: String = erroneous_components
/* FP:config.rs-2771 */             .into_iter()
/* FP:config.rs-2772 */             .map(|c| c.as_str().unwrap())
/* FP:config.rs-2773 */             .intersperse(", ")
/* FP:config.rs-2774 */             .collect();
/* FP:config.rs-2775 */         early_dcx.early_fatal(format!(
/* FP:config.rs-2776 */             "some `-C link-self-contained` components were both enabled and disabled: {names}"
/* FP:config.rs-2777 */         ));
/* FP:config.rs-2778 */     }
/* FP:config.rs-2779 */ 
/* FP:config.rs-2780 */     let prints = collect_print_requests(early_dcx, &mut cg, &unstable_opts, matches);
/* FP:config.rs-2781 */ 
/* FP:config.rs-2782 */     // -Zretpoline-external-thunk also requires -Zretpoline
/* FP:config.rs-2783 */     if unstable_opts.retpoline_external_thunk {
/* FP:config.rs-2784 */         unstable_opts.retpoline = true;
/* FP:config.rs-2785 */         target_modifiers.insert(
/* FP:config.rs-2786 */             OptionsTargetModifiers::UnstableOptions(UnstableOptionsTargetModifiers::retpoline),
/* FP:config.rs-2787 */             "true".to_string(),
/* FP:config.rs-2788 */         );
/* FP:config.rs-2789 */     }
/* FP:config.rs-2790 */ 
/* FP:config.rs-2791 */     let cg = cg;
/* FP:config.rs-2792 */ 
/* FP:config.rs-2793 */     let opt_level = parse_opt_level(early_dcx, matches, &cg);
/* FP:config.rs-2794 */     // The `-g` and `-C debuginfo` flags specify the same setting, so we want to be able
/* FP:config.rs-2795 */     // to use them interchangeably. See the note above (regarding `-O` and `-C opt-level`)
/* FP:config.rs-2796 */     // for more details.
/* FP:config.rs-2797 */     let debug_assertions = cg.debug_assertions.unwrap_or(opt_level == OptLevel::No);
/* FP:config.rs-2798 */     let debuginfo = select_debuginfo(matches, &cg);
/* FP:config.rs-2799 */     let debuginfo_compression = unstable_opts.debuginfo_compression;
/* FP:config.rs-2800 */ 
/* FP:config.rs-2801 */     if !unstable_options_enabled {
/* FP:config.rs-2802 */         if let Err(error) = cg.linker_features.check_unstable_variants(&target_triple) {
/* FP:config.rs-2803 */             early_dcx.early_fatal(error);
/* FP:config.rs-2804 */         }
/* FP:config.rs-2805 */     }
/* FP:config.rs-2806 */ 
/* FP:config.rs-2807 */     let crate_name = matches.opt_str("crate-name");
/* FP:config.rs-2808 */     let unstable_features = UnstableFeatures::from_environment(crate_name.as_deref());
/* FP:config.rs-2809 */     // Parse any `-l` flags, which link to native libraries.
/* FP:config.rs-2810 */     let libs = parse_native_libs(early_dcx, &unstable_opts, unstable_features, matches);
/* FP:config.rs-2811 */ 
/* FP:config.rs-2812 */     let test = matches.opt_present("test");
/* FP:config.rs-2813 */ 
/* FP:config.rs-2814 */     if !cg.remark.is_empty() && debuginfo == DebugInfo::None {
/* FP:config.rs-2815 */         early_dcx.early_warn("-C remark requires \"-C debuginfo=n\" to show source locations");
/* FP:config.rs-2816 */     }
/* FP:config.rs-2817 */ 
/* FP:config.rs-2818 */     if cg.remark.is_empty() && unstable_opts.remark_dir.is_some() {
/* FP:config.rs-2819 */         early_dcx
/* FP:config.rs-2820 */             .early_warn("using -Z remark-dir without enabling remarks using e.g. -C remark=all");
/* FP:config.rs-2821 */     }
/* FP:config.rs-2822 */ 
/* FP:config.rs-2823 */     let externs = parse_externs(early_dcx, matches, &unstable_opts);
/* FP:config.rs-2824 */ 
/* FP:config.rs-2825 */     let remap_path_prefix = parse_remap_path_prefix(early_dcx, matches, &unstable_opts);
/* FP:config.rs-2826 */ 
/* FP:config.rs-2827 */     let pretty = parse_pretty(early_dcx, &unstable_opts);
/* FP:config.rs-2828 */ 
/* FP:config.rs-2829 */     // query-dep-graph is required if dump-dep-graph is given #106736
/* FP:config.rs-2830 */     if unstable_opts.dump_dep_graph && !unstable_opts.query_dep_graph {
/* FP:config.rs-2831 */         early_dcx.early_fatal("can't dump dependency graph without `-Z query-dep-graph`");
/* FP:config.rs-2832 */     }
/* FP:config.rs-2833 */ 
/* FP:config.rs-2834 */     let logical_env = parse_logical_env(early_dcx, matches);
/* FP:config.rs-2835 */ 
/* FP:config.rs-2836 */     let sysroot = Sysroot::new(matches.opt_str("sysroot").map(PathBuf::from));
/* FP:config.rs-2837 */ 
/* FP:config.rs-2838 */     let real_source_base_dir = |suffix: &str, confirm: &str| {
/* FP:config.rs-2839 */         let mut candidate = sysroot.path().join(suffix);
/* FP:config.rs-2840 */         if let Ok(metadata) = candidate.symlink_metadata() {
/* FP:config.rs-2841 */             // Replace the symlink bootstrap creates, with its destination.
/* FP:config.rs-2842 */             // We could try to use `fs::canonicalize` instead, but that might
/* FP:config.rs-2843 */             // produce unnecessarily verbose path.
/* FP:config.rs-2844 */             if metadata.file_type().is_symlink() {
/* FP:config.rs-2845 */                 if let Ok(symlink_dest) = std::fs::read_link(&candidate) {
/* FP:config.rs-2846 */                     candidate = symlink_dest;
/* FP:config.rs-2847 */                 }
/* FP:config.rs-2848 */             }
/* FP:config.rs-2849 */         }
/* FP:config.rs-2850 */ 
/* FP:config.rs-2851 */         // Only use this directory if it has a file we can expect to always find.
/* FP:config.rs-2852 */         candidate.join(confirm).is_file().then_some(candidate)
/* FP:config.rs-2853 */     };
/* FP:config.rs-2854 */ 
/* FP:config.rs-2855 */     let real_rust_source_base_dir =
/* FP:config.rs-2856 */         // This is the location used by the `rust-src` `rustup` component.
/* FP:config.rs-2857 */         real_source_base_dir("lib/rustlib/src/rust", "library/std/src/lib.rs");
/* FP:config.rs-2858 */ 
/* FP:config.rs-2859 */     let real_rustc_dev_source_base_dir =
/* FP:config.rs-2860 */         // This is the location used by the `rustc-dev` `rustup` component.
/* FP:config.rs-2861 */         real_source_base_dir("lib/rustlib/rustc-src/rust", "compiler/rustc/src/main.rs");
/* FP:config.rs-2862 */ 
/* FP:config.rs-2863 */     // We eagerly scan all files in each passed -L path. If the same directory is passed multiple
/* FP:config.rs-2864 */     // times, and the directory contains a lot of files, this can take a lot of time.
/* FP:config.rs-2865 */     // So we remove -L paths that were passed multiple times, and keep only the first occurrence.
/* FP:config.rs-2866 */     // We still have to keep the original order of the -L arguments.
/* FP:config.rs-2867 */     let search_paths: Vec<SearchPath> = {
/* FP:config.rs-2868 */         let mut seen_search_paths = FxHashSet::default();
/* FP:config.rs-2869 */         let search_path_matches: Vec<String> = matches.opt_strs("L");
/* FP:config.rs-2870 */         search_path_matches
/* FP:config.rs-2871 */             .iter()
/* FP:config.rs-2872 */             .filter(|p| seen_search_paths.insert(*p))
/* FP:config.rs-2873 */             .map(|path| {
/* FP:config.rs-2874 */                 SearchPath::from_cli_opt(
/* FP:config.rs-2875 */                     sysroot.path(),
/* FP:config.rs-2876 */                     &target_triple,
/* FP:config.rs-2877 */                     early_dcx,
/* FP:config.rs-2878 */                     &path,
/* FP:config.rs-2879 */                     unstable_opts.unstable_options,
/* FP:config.rs-2880 */                 )
/* FP:config.rs-2881 */             })
/* FP:config.rs-2882 */             .collect()
/* FP:config.rs-2883 */     };
/* FP:config.rs-2884 */ 
/* FP:config.rs-2885 */     let working_dir = std::env::current_dir().unwrap_or_else(|e| {
/* FP:config.rs-2886 */         early_dcx.early_fatal(format!("Current directory is invalid: {e}"));
/* FP:config.rs-2887 */     });
/* FP:config.rs-2888 */ 
/* FP:config.rs-2889 */     let file_mapping = file_path_mapping(remap_path_prefix.clone(), &unstable_opts);
/* FP:config.rs-2890 */     let working_dir = file_mapping.to_real_filename(&working_dir);
/* FP:config.rs-2891 */ 
/* FP:config.rs-2892 */     let verbose = matches.opt_present("verbose") || unstable_opts.verbose_internals;
/* FP:config.rs-2893 */ 
/* FP:config.rs-2894 */     Options {
/* FP:config.rs-2895 */         assert_incr_state,
/* FP:config.rs-2896 */         crate_types,
/* FP:config.rs-2897 */         optimize: opt_level,
/* FP:config.rs-2898 */         debuginfo,
/* FP:config.rs-2899 */         debuginfo_compression,
/* FP:config.rs-2900 */         lint_opts,
/* FP:config.rs-2901 */         lint_cap,
/* FP:config.rs-2902 */         describe_lints,
/* FP:config.rs-2903 */         output_types,
/* FP:config.rs-2904 */         search_paths,
/* FP:config.rs-2905 */         sysroot,
/* FP:config.rs-2906 */         target_triple,
/* FP:config.rs-2907 */         test,
/* FP:config.rs-2908 */         incremental,
/* FP:config.rs-2909 */         untracked_state_hash: Default::default(),
/* FP:config.rs-2910 */         unstable_opts,
/* FP:config.rs-2911 */         prints,
/* FP:config.rs-2912 */         cg,
/* FP:config.rs-2913 */         error_format,
/* FP:config.rs-2914 */         diagnostic_width,
/* FP:config.rs-2915 */         externs,
/* FP:config.rs-2916 */         unstable_features,
/* FP:config.rs-2917 */         crate_name,
/* FP:config.rs-2918 */         libs,
/* FP:config.rs-2919 */         debug_assertions,
/* FP:config.rs-2920 */         actually_rustdoc: false,
/* FP:config.rs-2921 */         resolve_doc_links: ResolveDocLinks::ExportedMetadata,
/* FP:config.rs-2922 */         trimmed_def_paths: false,
/* FP:config.rs-2923 */         cli_forced_codegen_units: codegen_units,
/* FP:config.rs-2924 */         cli_forced_local_thinlto_off: disable_local_thinlto,
/* FP:config.rs-2925 */         remap_path_prefix,
/* FP:config.rs-2926 */         real_rust_source_base_dir,
/* FP:config.rs-2927 */         real_rustc_dev_source_base_dir,
/* FP:config.rs-2928 */         edition,
/* FP:config.rs-2929 */         json_artifact_notifications,
/* FP:config.rs-2930 */         json_timings,
/* FP:config.rs-2931 */         json_unused_externs,
/* FP:config.rs-2932 */         json_future_incompat,
/* FP:config.rs-2933 */         pretty,
/* FP:config.rs-2934 */         working_dir,
/* FP:config.rs-2935 */         color,
/* FP:config.rs-2936 */         logical_env,
/* FP:config.rs-2937 */         verbose,
/* FP:config.rs-2938 */         target_modifiers,
/* FP:config.rs-2939 */     }
/* FP:config.rs-2940 */ }
/* FP:config.rs-2941 */ 
/* FP:config.rs-2942 */ fn parse_pretty(early_dcx: &EarlyDiagCtxt, unstable_opts: &UnstableOptions) -> Option<PpMode> {
/* FP:config.rs-2943 */     use PpMode::*;
/* FP:config.rs-2944 */ 
/* FP:config.rs-2945 */     let first = match unstable_opts.unpretty.as_deref()? {
/* FP:config.rs-2946 */         "normal" => Source(PpSourceMode::Normal),
/* FP:config.rs-2947 */         "identified" => Source(PpSourceMode::Identified),
/* FP:config.rs-2948 */         "expanded" => Source(PpSourceMode::Expanded),
/* FP:config.rs-2949 */         "expanded,identified" => Source(PpSourceMode::ExpandedIdentified),
/* FP:config.rs-2950 */         "expanded,hygiene" => Source(PpSourceMode::ExpandedHygiene),
/* FP:config.rs-2951 */         "ast-tree" => AstTree,
/* FP:config.rs-2952 */         "ast-tree,expanded" => AstTreeExpanded,
/* FP:config.rs-2953 */         "hir" => Hir(PpHirMode::Normal),
/* FP:config.rs-2954 */         "hir,identified" => Hir(PpHirMode::Identified),
/* FP:config.rs-2955 */         "hir,typed" => Hir(PpHirMode::Typed),
/* FP:config.rs-2956 */         "hir-tree" => HirTree,
/* FP:config.rs-2957 */         "thir-tree" => ThirTree,
/* FP:config.rs-2958 */         "thir-flat" => ThirFlat,
/* FP:config.rs-2959 */         "mir" => Mir,
/* FP:config.rs-2960 */         "stable-mir" => StableMir,
/* FP:config.rs-2961 */         "mir-cfg" => MirCFG,
/* FP:config.rs-2962 */         name => early_dcx.early_fatal(format!(
/* FP:config.rs-2963 */             "argument to `unpretty` must be one of `normal`, `identified`, \
/* FP:config.rs-2964 */                             `expanded`, `expanded,identified`, `expanded,hygiene`, \
/* FP:config.rs-2965 */                             `ast-tree`, `ast-tree,expanded`, `hir`, `hir,identified`, \
/* FP:config.rs-2966 */                             `hir,typed`, `hir-tree`, `thir-tree`, `thir-flat`, `mir`, `stable-mir`, or \
/* FP:config.rs-2967 */                             `mir-cfg`; got {name}"
/* FP:config.rs-2968 */         )),
/* FP:config.rs-2969 */     };
/* FP:config.rs-2970 */     debug!("got unpretty option: {first:?}");
/* FP:config.rs-2971 */     Some(first)
/* FP:config.rs-2972 */ }
/* FP:config.rs-2973 */ 
/* FP:config.rs-2974 */ pub fn make_crate_type_option() -> RustcOptGroup {
/* FP:config.rs-2975 */     make_opt(
/* FP:config.rs-2976 */         OptionStability::Stable,
/* FP:config.rs-2977 */         OptionKind::Multi,
/* FP:config.rs-2978 */         "",
/* FP:config.rs-2979 */         "crate-type",
/* FP:config.rs-2980 */         "Comma separated list of types of crates
/* FP:config.rs-2981 */                                 for the compiler to emit",
/* FP:config.rs-2982 */         "<bin|lib|rlib|dylib|cdylib|staticlib|proc-macro>",
/* FP:config.rs-2983 */     )
/* FP:config.rs-2984 */ }
/* FP:config.rs-2985 */ 
/* FP:config.rs-2986 */ pub fn parse_crate_types_from_list(list_list: Vec<String>) -> Result<Vec<CrateType>, String> {
/* FP:config.rs-2987 */     let mut crate_types: Vec<CrateType> = Vec::new();
/* FP:config.rs-2988 */     for unparsed_crate_type in &list_list {
/* FP:config.rs-2989 */         for part in unparsed_crate_type.split(',') {
/* FP:config.rs-2990 */             let new_part = match part {
/* FP:config.rs-2991 */                 "lib" => default_lib_output(),
/* FP:config.rs-2992 */                 "rlib" => CrateType::Rlib,
/* FP:config.rs-2993 */                 "staticlib" => CrateType::Staticlib,
/* FP:config.rs-2994 */                 "dylib" => CrateType::Dylib,
/* FP:config.rs-2995 */                 "cdylib" => CrateType::Cdylib,
/* FP:config.rs-2996 */                 "bin" => CrateType::Executable,
/* FP:config.rs-2997 */                 "proc-macro" => CrateType::ProcMacro,
/* FP:config.rs-2998 */                 "sdylib" => CrateType::Sdylib,
/* FP:config.rs-2999 */                 _ => {
/* FP:config.rs-3000 */                     return Err(format!(
/* FP:config.rs-3001 */                         "unknown crate type: `{part}`, expected one of: \
/* FP:config.rs-3002 */                         `lib`, `rlib`, `staticlib`, `dylib`, `cdylib`, `bin`, `proc-macro`",
/* FP:config.rs-3003 */                     ));
/* FP:config.rs-3004 */                 }
/* FP:config.rs-3005 */             };
/* FP:config.rs-3006 */             if !crate_types.contains(&new_part) {
/* FP:config.rs-3007 */                 crate_types.push(new_part)
/* FP:config.rs-3008 */             }
/* FP:config.rs-3009 */         }
/* FP:config.rs-3010 */     }
/* FP:config.rs-3011 */ 
/* FP:config.rs-3012 */     Ok(crate_types)
/* FP:config.rs-3013 */ }
/* FP:config.rs-3014 */ 
/* FP:config.rs-3015 */ pub mod nightly_options {
/* FP:config.rs-3016 */     use crate::rustc_feature::UnstableFeatures;
/* FP:config.rs-3017 */ 
/* FP:config.rs-3018 */     use super::{OptionStability, RustcOptGroup};
/* FP:config.rs-3019 */     use crate::EarlyDiagCtxt;
/* FP:config.rs-3020 */ 
/* FP:config.rs-3021 */     pub fn is_unstable_enabled(matches: &getopts::Matches) -> bool {
/* FP:config.rs-3022 */         match_is_nightly_build(matches)
/* FP:config.rs-3023 */             && matches.opt_strs("Z").iter().any(|x| *x == "unstable-options")
/* FP:config.rs-3024 */     }
/* FP:config.rs-3025 */ 
/* FP:config.rs-3026 */     pub fn match_is_nightly_build(matches: &getopts::Matches) -> bool {
/* FP:config.rs-3027 */         is_nightly_build(matches.opt_str("crate-name").as_deref())
/* FP:config.rs-3028 */     }
/* FP:config.rs-3029 */ 
/* FP:config.rs-3030 */     fn is_nightly_build(krate: Option<&str>) -> bool {
/* FP:config.rs-3031 */         UnstableFeatures::from_environment(krate).is_nightly_build()
/* FP:config.rs-3032 */     }
/* FP:config.rs-3033 */ 
/* FP:config.rs-3034 */     pub fn check_nightly_options(
/* FP:config.rs-3035 */         early_dcx: &EarlyDiagCtxt,
/* FP:config.rs-3036 */         matches: &getopts::Matches,
/* FP:config.rs-3037 */         flags: &[RustcOptGroup],
/* FP:config.rs-3038 */     ) {
/* FP:config.rs-3039 */         let has_z_unstable_option = matches.opt_strs("Z").iter().any(|x| *x == "unstable-options");
/* FP:config.rs-3040 */         let really_allows_unstable_options = match_is_nightly_build(matches);
/* FP:config.rs-3041 */         let mut nightly_options_on_stable = 0;
/* FP:config.rs-3042 */ 
/* FP:config.rs-3043 */         for opt in flags.iter() {
/* FP:config.rs-3044 */             if opt.stability == OptionStability::Stable {
/* FP:config.rs-3045 */                 continue;
/* FP:config.rs-3046 */             }
/* FP:config.rs-3047 */             if !matches.opt_present(opt.name) {
/* FP:config.rs-3048 */                 continue;
/* FP:config.rs-3049 */             }
/* FP:config.rs-3050 */             if opt.name != "Z" && !has_z_unstable_option {
/* FP:config.rs-3051 */                 early_dcx.early_fatal(format!(
/* FP:config.rs-3052 */                     "the `-Z unstable-options` flag must also be passed to enable \
/* FP:config.rs-3053 */                          the flag `{}`",
/* FP:config.rs-3054 */                     opt.name
/* FP:config.rs-3055 */                 ));
/* FP:config.rs-3056 */             }
/* FP:config.rs-3057 */             if really_allows_unstable_options {
/* FP:config.rs-3058 */                 continue;
/* FP:config.rs-3059 */             }
/* FP:config.rs-3060 */             match opt.stability {
/* FP:config.rs-3061 */                 OptionStability::Unstable => {
/* FP:config.rs-3062 */                     nightly_options_on_stable += 1;
/* FP:config.rs-3063 */                     let msg = format!(
/* FP:config.rs-3064 */                         "the option `{}` is only accepted on the nightly compiler",
/* FP:config.rs-3065 */                         opt.name
/* FP:config.rs-3066 */                     );
/* FP:config.rs-3067 */                     // The non-zero nightly_options_on_stable will force an early_fatal eventually.
/* FP:config.rs-3068 */                     let _ = early_dcx.early_err(msg);
/* FP:config.rs-3069 */                 }
/* FP:config.rs-3070 */                 OptionStability::Stable => {}
/* FP:config.rs-3071 */             }
/* FP:config.rs-3072 */         }
/* FP:config.rs-3073 */         if nightly_options_on_stable > 0 {
/* FP:config.rs-3074 */             early_dcx
/* FP:config.rs-3075 */                 .early_help("consider switching to a nightly toolchain: `rustup default nightly`");
/* FP:config.rs-3076 */             early_dcx.early_note("selecting a toolchain with `+toolchain` arguments require a rustup proxy; see <https://rust-lang.github.io/rustup/concepts/index.html>");
/* FP:config.rs-3077 */             early_dcx.early_note("for more information about Rust's stability policy, see <https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#unstable-features>");
/* FP:config.rs-3078 */             early_dcx.early_fatal(format!(
/* FP:config.rs-3079 */                 "{} nightly option{} were parsed",
/* FP:config.rs-3080 */                 nightly_options_on_stable,
/* FP:config.rs-3081 */                 if nightly_options_on_stable > 1 { "s" } else { "" }
/* FP:config.rs-3082 */             ));
/* FP:config.rs-3083 */         }
/* FP:config.rs-3084 */     }
/* FP:config.rs-3085 */ }
/* FP:config.rs-3086 */ 
/* FP:config.rs-3087 */ impl fmt::Display for CrateType {
/* FP:config.rs-3088 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:config.rs-3089 */         match *self {
/* FP:config.rs-3090 */             CrateType::Executable => "bin".fmt(f),
/* FP:config.rs-3091 */             CrateType::Dylib => "dylib".fmt(f),
/* FP:config.rs-3092 */             CrateType::Rlib => "rlib".fmt(f),
/* FP:config.rs-3093 */             CrateType::Staticlib => "staticlib".fmt(f),
/* FP:config.rs-3094 */             CrateType::Cdylib => "cdylib".fmt(f),
/* FP:config.rs-3095 */             CrateType::ProcMacro => "proc-macro".fmt(f),
/* FP:config.rs-3096 */             CrateType::Sdylib => "sdylib".fmt(f),
/* FP:config.rs-3097 */         }
/* FP:config.rs-3098 */     }
/* FP:config.rs-3099 */ }
/* FP:config.rs-3100 */ 
/* FP:config.rs-3101 */ impl IntoDiagArg for CrateType {
/* FP:config.rs-3102 */     fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
/* FP:config.rs-3103 */         self.to_string().into_diag_arg(&mut None)
/* FP:config.rs-3104 */     }
/* FP:config.rs-3105 */ }
/* FP:config.rs-3106 */ 
/* FP:config.rs-3107 */ #[derive(Copy, Clone, PartialEq, Debug)]
/* FP:config.rs-3108 */ pub enum PpSourceMode {
/* FP:config.rs-3109 */     /// `-Zunpretty=normal`
/* FP:config.rs-3110 */     Normal,
/* FP:config.rs-3111 */     /// `-Zunpretty=expanded`
/* FP:config.rs-3112 */     Expanded,
/* FP:config.rs-3113 */     /// `-Zunpretty=identified`
/* FP:config.rs-3114 */     Identified,
/* FP:config.rs-3115 */     /// `-Zunpretty=expanded,identified`
/* FP:config.rs-3116 */     ExpandedIdentified,
/* FP:config.rs-3117 */     /// `-Zunpretty=expanded,hygiene`
/* FP:config.rs-3118 */     ExpandedHygiene,
/* FP:config.rs-3119 */ }
/* FP:config.rs-3120 */ 
/* FP:config.rs-3121 */ #[derive(Copy, Clone, PartialEq, Debug)]
/* FP:config.rs-3122 */ pub enum PpHirMode {
/* FP:config.rs-3123 */     /// `-Zunpretty=hir`
/* FP:config.rs-3124 */     Normal,
/* FP:config.rs-3125 */     /// `-Zunpretty=hir,identified`
/* FP:config.rs-3126 */     Identified,
/* FP:config.rs-3127 */     /// `-Zunpretty=hir,typed`
/* FP:config.rs-3128 */     Typed,
/* FP:config.rs-3129 */ }
/* FP:config.rs-3130 */ 
/* FP:config.rs-3131 */ #[derive(Copy, Clone, PartialEq, Debug)]
/* FP:config.rs-3132 */ /// Pretty print mode
/* FP:config.rs-3133 */ pub enum PpMode {
/* FP:config.rs-3134 */     /// Options that print the source code, i.e.
/* FP:config.rs-3135 */     /// `-Zunpretty=normal` and `-Zunpretty=expanded`
/* FP:config.rs-3136 */     Source(PpSourceMode),
/* FP:config.rs-3137 */     /// `-Zunpretty=ast-tree`
/* FP:config.rs-3138 */     AstTree,
/* FP:config.rs-3139 */     /// `-Zunpretty=ast-tree,expanded`
/* FP:config.rs-3140 */     AstTreeExpanded,
/* FP:config.rs-3141 */     /// Options that print the HIR, i.e. `-Zunpretty=hir`
/* FP:config.rs-3142 */     Hir(PpHirMode),
/* FP:config.rs-3143 */     /// `-Zunpretty=hir-tree`
/* FP:config.rs-3144 */     HirTree,
/* FP:config.rs-3145 */     /// `-Zunpretty=thir-tree`
/* FP:config.rs-3146 */     ThirTree,
/* FP:config.rs-3147 */     /// `-Zunpretty=thir-flat`
/* FP:config.rs-3148 */     ThirFlat,
/* FP:config.rs-3149 */     /// `-Zunpretty=mir`
/* FP:config.rs-3150 */     Mir,
/* FP:config.rs-3151 */     /// `-Zunpretty=mir-cfg`
/* FP:config.rs-3152 */     MirCFG,
/* FP:config.rs-3153 */     /// `-Zunpretty=stable-mir`
/* FP:config.rs-3154 */     StableMir,
/* FP:config.rs-3155 */ }
/* FP:config.rs-3156 */ 
/* FP:config.rs-3157 */ impl PpMode {
/* FP:config.rs-3158 */     pub fn needs_ast_map(&self) -> bool {
/* FP:config.rs-3159 */         use PpMode::*;
/* FP:config.rs-3160 */         use PpSourceMode::*;
/* FP:config.rs-3161 */         match *self {
/* FP:config.rs-3162 */             Source(Normal | Identified) | AstTree => false,
/* FP:config.rs-3163 */ 
/* FP:config.rs-3164 */             Source(Expanded | ExpandedIdentified | ExpandedHygiene)
/* FP:config.rs-3165 */             | AstTreeExpanded
/* FP:config.rs-3166 */             | Hir(_)
/* FP:config.rs-3167 */             | HirTree
/* FP:config.rs-3168 */             | ThirTree
/* FP:config.rs-3169 */             | ThirFlat
/* FP:config.rs-3170 */             | Mir
/* FP:config.rs-3171 */             | MirCFG
/* FP:config.rs-3172 */             | StableMir => true,
/* FP:config.rs-3173 */         }
/* FP:config.rs-3174 */     }
/* FP:config.rs-3175 */ 
/* FP:config.rs-3176 */     pub fn needs_analysis(&self) -> bool {
/* FP:config.rs-3177 */         use PpMode::*;
/* FP:config.rs-3178 */         matches!(*self, Hir(PpHirMode::Typed) | Mir | StableMir | MirCFG | ThirTree | ThirFlat)
/* FP:config.rs-3179 */     }
/* FP:config.rs-3180 */ }
/* FP:config.rs-3181 */ 
/* FP:config.rs-3182 */ #[derive(Clone, Hash, PartialEq, Eq, Debug)]
/* FP:config.rs-3183 */ pub enum WasiExecModel {
/* FP:config.rs-3184 */     Command,
/* FP:config.rs-3185 */     Reactor,
/* FP:config.rs-3186 */ }
/* FP:config.rs-3187 */ 
/* FP:config.rs-3188 */ /// Command-line arguments passed to the compiler have to be incorporated with
/* FP:config.rs-3189 */ /// the dependency tracking system for incremental compilation. This module
/* FP:config.rs-3190 */ /// provides some utilities to make this more convenient.
/* FP:config.rs-3191 */ ///
/* FP:config.rs-3192 */ /// The values of all command-line arguments that are relevant for dependency
/* FP:config.rs-3193 */ /// tracking are hashed into a single value that determines whether the
/* FP:config.rs-3194 */ /// incremental compilation cache can be re-used or not. This hashing is done
/* FP:config.rs-3195 */ /// via the `DepTrackingHash` trait defined below, since the standard `Hash`
/* FP:config.rs-3196 */ /// implementation might not be suitable (e.g., arguments are stored in a `Vec`,
/* FP:config.rs-3197 */ /// the hash of which is order dependent, but we might not want the order of
/* FP:config.rs-3198 */ /// arguments to make a difference for the hash).
/* FP:config.rs-3199 */ ///
/* FP:config.rs-3200 */ /// However, since the value provided by `Hash::hash` often *is* suitable,
/* FP:config.rs-3201 */ /// especially for primitive types, there is the
/* FP:config.rs-3202 */ /// `impl_dep_tracking_hash_via_hash!()` macro that allows to simply reuse the
/* FP:config.rs-3203 */ /// `Hash` implementation for `DepTrackingHash`. It's important though that
/* FP:config.rs-3204 */ /// we have an opt-in scheme here, so one is hopefully forced to think about
/* FP:config.rs-3205 */ /// how the hash should be calculated when adding a new command-line argument.
/* FP:config.rs-3206 */ pub(crate) mod dep_tracking {
/* FP:config.rs-3207 */     use std::collections::BTreeMap;
/* FP:config.rs-3208 */     use std::hash::Hash;
/* FP:config.rs-3209 */     use std::num::NonZero;
/* FP:config.rs-3210 */     use std::path::PathBuf;
/* FP:config.rs-3211 */ 
/* FP:config.rs-3212 */     use crate::rustc_abi::Align;
/* FP:config.rs-3213 */     use crate::rustc_data_structures::fx::FxIndexMap;
/* FP:config.rs-3214 */     use crate::rustc_data_structures::stable_hasher::StableHasher;
/* FP:config.rs-3215 */     use crate::rustc_complete::LanguageIdentifier;
/* FP:config.rs-3216 */     use crate::rustc_feature::UnstableFeatures;
/* FP:config.rs-3217 */     use rustc_hashes::Hash64;
/* FP:config.rs-3218 */     use crate::rustc_complete::RealFileName;
/* FP:config.rs-3219 */     use crate::rustc_complete::edition::Edition;
/* FP:config.rs-3220 */     use crate::rustc_target::spec::{
/* FP:config.rs-3221 */         CodeModel, FramePointer, MergeFunctions, OnBrokenPipe, PanicStrategy, RelocModel,
/* FP:config.rs-3222 */         RelroLevel, SanitizerSet, SplitDebuginfo, StackProtector, SymbolVisibility, TargetTuple,
/* FP:config.rs-3223 */         TlsModel,
/* FP:config.rs-3224 */     };
/* FP:config.rs-3225 */ 
/* FP:config.rs-3226 */     use super::{
/* FP:config.rs-3227 */         AutoDiff, BranchProtection, CFGuard, CFProtection, CollapseMacroDebuginfo, CoverageOptions,
/* FP:config.rs-3228 */         CrateType, DebugInfo, DebugInfoCompression, ErrorOutputType, FmtDebug, FunctionReturn,
/* FP:config.rs-3229 */         InliningThreshold, InstrumentCoverage, InstrumentXRay, LinkerPluginLto, LocationDetail,
/* FP:config.rs-3230 */         LtoCli, MirStripDebugInfo, NextSolverConfig, Offload, OomStrategy, OptLevel, OutFileName,
/* FP:config.rs-3231 */         OutputType, OutputTypes, PatchableFunctionEntry, Polonius, RemapPathScopeComponents,
/* FP:config.rs-3232 */         ResolveDocLinks, SourceFileHashAlgorithm, SplitDwarfKind, SwitchWithOptPath,
/* FP:config.rs-3233 */         SymbolManglingVersion, WasiExecModel,
/* FP:config.rs-3234 */     };
/* FP:config.rs-3235 */     use crate::lint;
/* FP:config.rs-3236 */     use crate::utils::NativeLib;
/* FP:config.rs-3237 */ 
/* FP:config.rs-3238 */     pub(crate) trait DepTrackingHash {
/* FP:config.rs-3239 */         fn hash(
/* FP:config.rs-3240 */             &self,
/* FP:config.rs-3241 */             hasher: &mut StableHasher,
/* FP:config.rs-3242 */             error_format: ErrorOutputType,
/* FP:config.rs-3243 */             for_crate_hash: bool,
/* FP:config.rs-3244 */         );
/* FP:config.rs-3245 */     }
/* FP:config.rs-3246 */ 
/* FP:config.rs-3247 */     macro_rules! impl_dep_tracking_hash_via_hash {
/* FP:config.rs-3248 */         ($($t:ty),+ $(,)?) => {$(
/* FP:config.rs-3249 */             impl DepTrackingHash for $t {
/* FP:config.rs-3250 */                 fn hash(&self, hasher: &mut StableHasher, _: ErrorOutputType, _for_crate_hash: bool) {
/* FP:config.rs-3251 */                     Hash::hash(self, hasher);
/* FP:config.rs-3252 */                 }
/* FP:config.rs-3253 */             }
/* FP:config.rs-3254 */         )+};
/* FP:config.rs-3255 */     }
/* FP:config.rs-3256 */ 
/* FP:config.rs-3257 */     impl<T: DepTrackingHash> DepTrackingHash for Option<T> {
/* FP:config.rs-3258 */         fn hash(
/* FP:config.rs-3259 */             &self,
/* FP:config.rs-3260 */             hasher: &mut StableHasher,
/* FP:config.rs-3261 */             error_format: ErrorOutputType,
/* FP:config.rs-3262 */             for_crate_hash: bool,
/* FP:config.rs-3263 */         ) {
/* FP:config.rs-3264 */             match self {
/* FP:config.rs-3265 */                 Some(x) => {
/* FP:config.rs-3266 */                     Hash::hash(&1, hasher);
/* FP:config.rs-3267 */                     DepTrackingHash::hash(x, hasher, error_format, for_crate_hash);
/* FP:config.rs-3268 */                 }
/* FP:config.rs-3269 */                 None => Hash::hash(&0, hasher),
/* FP:config.rs-3270 */             }
/* FP:config.rs-3271 */         }
/* FP:config.rs-3272 */     }
/* FP:config.rs-3273 */ 
/* FP:config.rs-3274 */     impl_dep_tracking_hash_via_hash!(
/* FP:config.rs-3275 */         (),
/* FP:config.rs-3276 */         AutoDiff,
/* FP:config.rs-3277 */         Offload,
/* FP:config.rs-3278 */         bool,
/* FP:config.rs-3279 */         usize,
/* FP:config.rs-3280 */         NonZero<usize>,
/* FP:config.rs-3281 */         u64,
/* FP:config.rs-3282 */         Hash64,
/* FP:config.rs-3283 */         String,
/* FP:config.rs-3284 */         PathBuf,
/* FP:config.rs-3285 */         lint::Level,
/* FP:config.rs-3286 */         WasiExecModel,
/* FP:config.rs-3287 */         u32,
/* FP:config.rs-3288 */         FramePointer,
/* FP:config.rs-3289 */         RelocModel,
/* FP:config.rs-3290 */         CodeModel,
/* FP:config.rs-3291 */         TlsModel,
/* FP:config.rs-3292 */         InstrumentCoverage,
/* FP:config.rs-3293 */         CoverageOptions,
/* FP:config.rs-3294 */         InstrumentXRay,
/* FP:config.rs-3295 */         CrateType,
/* FP:config.rs-3296 */         MergeFunctions,
/* FP:config.rs-3297 */         OnBrokenPipe,
/* FP:config.rs-3298 */         PanicStrategy,
/* FP:config.rs-3299 */         RelroLevel,
/* FP:config.rs-3300 */         OptLevel,
/* FP:config.rs-3301 */         LtoCli,
/* FP:config.rs-3302 */         DebugInfo,
/* FP:config.rs-3303 */         DebugInfoCompression,
/* FP:config.rs-3304 */         MirStripDebugInfo,
/* FP:config.rs-3305 */         CollapseMacroDebuginfo,
/* FP:config.rs-3306 */         UnstableFeatures,
/* FP:config.rs-3307 */         NativeLib,
/* FP:config.rs-3308 */         SanitizerSet,
/* FP:config.rs-3309 */         CFGuard,
/* FP:config.rs-3310 */         CFProtection,
/* FP:config.rs-3311 */         TargetTuple,
/* FP:config.rs-3312 */         Edition,
/* FP:config.rs-3313 */         LinkerPluginLto,
/* FP:config.rs-3314 */         ResolveDocLinks,
/* FP:config.rs-3315 */         SplitDebuginfo,
/* FP:config.rs-3316 */         SplitDwarfKind,
/* FP:config.rs-3317 */         StackProtector,
/* FP:config.rs-3318 */         SwitchWithOptPath,
/* FP:config.rs-3319 */         SymbolManglingVersion,
/* FP:config.rs-3320 */         SymbolVisibility,
/* FP:config.rs-3321 */         RemapPathScopeComponents,
/* FP:config.rs-3322 */         SourceFileHashAlgorithm,
/* FP:config.rs-3323 */         OutFileName,
/* FP:config.rs-3324 */         OutputType,
/* FP:config.rs-3325 */         RealFileName,
/* FP:config.rs-3326 */         LocationDetail,
/* FP:config.rs-3327 */         FmtDebug,
/* FP:config.rs-3328 */         BranchProtection,
/* FP:config.rs-3329 */         OomStrategy,
/* FP:config.rs-3330 */         LanguageIdentifier,
/* FP:config.rs-3331 */         NextSolverConfig,
/* FP:config.rs-3332 */         PatchableFunctionEntry,
/* FP:config.rs-3333 */         Polonius,
/* FP:config.rs-3334 */         InliningThreshold,
/* FP:config.rs-3335 */         FunctionReturn,
/* FP:config.rs-3336 */         Align,
/* FP:config.rs-3337 */     );
/* FP:config.rs-3338 */ 
/* FP:config.rs-3339 */     impl<T1, T2> DepTrackingHash for (T1, T2)
/* FP:config.rs-3340 */     where
/* FP:config.rs-3341 */         T1: DepTrackingHash,
/* FP:config.rs-3342 */         T2: DepTrackingHash,
/* FP:config.rs-3343 */     {
/* FP:config.rs-3344 */         fn hash(
/* FP:config.rs-3345 */             &self,
/* FP:config.rs-3346 */             hasher: &mut StableHasher,
/* FP:config.rs-3347 */             error_format: ErrorOutputType,
/* FP:config.rs-3348 */             for_crate_hash: bool,
/* FP:config.rs-3349 */         ) {
/* FP:config.rs-3350 */             Hash::hash(&0, hasher);
/* FP:config.rs-3351 */             DepTrackingHash::hash(&self.0, hasher, error_format, for_crate_hash);
/* FP:config.rs-3352 */             Hash::hash(&1, hasher);
/* FP:config.rs-3353 */             DepTrackingHash::hash(&self.1, hasher, error_format, for_crate_hash);
/* FP:config.rs-3354 */         }
/* FP:config.rs-3355 */     }
/* FP:config.rs-3356 */ 
/* FP:config.rs-3357 */     impl<T1, T2, T3> DepTrackingHash for (T1, T2, T3)
/* FP:config.rs-3358 */     where
/* FP:config.rs-3359 */         T1: DepTrackingHash,
/* FP:config.rs-3360 */         T2: DepTrackingHash,
/* FP:config.rs-3361 */         T3: DepTrackingHash,
/* FP:config.rs-3362 */     {
/* FP:config.rs-3363 */         fn hash(
/* FP:config.rs-3364 */             &self,
/* FP:config.rs-3365 */             hasher: &mut StableHasher,
/* FP:config.rs-3366 */             error_format: ErrorOutputType,
/* FP:config.rs-3367 */             for_crate_hash: bool,
/* FP:config.rs-3368 */         ) {
/* FP:config.rs-3369 */             Hash::hash(&0, hasher);
/* FP:config.rs-3370 */             DepTrackingHash::hash(&self.0, hasher, error_format, for_crate_hash);
/* FP:config.rs-3371 */             Hash::hash(&1, hasher);
/* FP:config.rs-3372 */             DepTrackingHash::hash(&self.1, hasher, error_format, for_crate_hash);
/* FP:config.rs-3373 */             Hash::hash(&2, hasher);
/* FP:config.rs-3374 */             DepTrackingHash::hash(&self.2, hasher, error_format, for_crate_hash);
/* FP:config.rs-3375 */         }
/* FP:config.rs-3376 */     }
/* FP:config.rs-3377 */ 
/* FP:config.rs-3378 */     impl<T: DepTrackingHash> DepTrackingHash for Vec<T> {
/* FP:config.rs-3379 */         fn hash(
/* FP:config.rs-3380 */             &self,
/* FP:config.rs-3381 */             hasher: &mut StableHasher,
/* FP:config.rs-3382 */             error_format: ErrorOutputType,
/* FP:config.rs-3383 */             for_crate_hash: bool,
/* FP:config.rs-3384 */         ) {
/* FP:config.rs-3385 */             Hash::hash(&self.len(), hasher);
/* FP:config.rs-3386 */             for (index, elem) in self.iter().enumerate() {
/* FP:config.rs-3387 */                 Hash::hash(&index, hasher);
/* FP:config.rs-3388 */                 DepTrackingHash::hash(elem, hasher, error_format, for_crate_hash);
/* FP:config.rs-3389 */             }
/* FP:config.rs-3390 */         }
/* FP:config.rs-3391 */     }
/* FP:config.rs-3392 */ 
/* FP:config.rs-3393 */     impl<T: DepTrackingHash, V: DepTrackingHash> DepTrackingHash for FxIndexMap<T, V> {
/* FP:config.rs-3394 */         fn hash(
/* FP:config.rs-3395 */             &self,
/* FP:config.rs-3396 */             hasher: &mut StableHasher,
/* FP:config.rs-3397 */             error_format: ErrorOutputType,
/* FP:config.rs-3398 */             for_crate_hash: bool,
/* FP:config.rs-3399 */         ) {
/* FP:config.rs-3400 */             Hash::hash(&self.len(), hasher);
/* FP:config.rs-3401 */             for (key, value) in self.iter() {
/* FP:config.rs-3402 */                 DepTrackingHash::hash(key, hasher, error_format, for_crate_hash);
/* FP:config.rs-3403 */                 DepTrackingHash::hash(value, hasher, error_format, for_crate_hash);
/* FP:config.rs-3404 */             }
/* FP:config.rs-3405 */         }
/* FP:config.rs-3406 */     }
/* FP:config.rs-3407 */ 
/* FP:config.rs-3408 */     impl DepTrackingHash for OutputTypes {
/* FP:config.rs-3409 */         fn hash(
/* FP:config.rs-3410 */             &self,
/* FP:config.rs-3411 */             hasher: &mut StableHasher,
/* FP:config.rs-3412 */             error_format: ErrorOutputType,
/* FP:config.rs-3413 */             for_crate_hash: bool,
/* FP:config.rs-3414 */         ) {
/* FP:config.rs-3415 */             Hash::hash(&self.0.len(), hasher);
/* FP:config.rs-3416 */             for (key, val) in &self.0 {
/* FP:config.rs-3417 */                 DepTrackingHash::hash(key, hasher, error_format, for_crate_hash);
/* FP:config.rs-3418 */                 if !for_crate_hash {
/* FP:config.rs-3419 */                     DepTrackingHash::hash(val, hasher, error_format, for_crate_hash);
/* FP:config.rs-3420 */                 }
/* FP:config.rs-3421 */             }
/* FP:config.rs-3422 */         }
/* FP:config.rs-3423 */     }
/* FP:config.rs-3424 */ 
/* FP:config.rs-3425 */     // This is a stable hash because BTreeMap is a sorted container
/* FP:config.rs-3426 */     pub(crate) fn stable_hash(
/* FP:config.rs-3427 */         sub_hashes: BTreeMap<&'static str, &dyn DepTrackingHash>,
/* FP:config.rs-3428 */         hasher: &mut StableHasher,
/* FP:config.rs-3429 */         error_format: ErrorOutputType,
/* FP:config.rs-3430 */         for_crate_hash: bool,
/* FP:config.rs-3431 */     ) {
/* FP:config.rs-3432 */         for (key, sub_hash) in sub_hashes {
/* FP:config.rs-3433 */             // Using Hash::hash() instead of DepTrackingHash::hash() is fine for
/* FP:config.rs-3434 */             // the keys, as they are just plain strings
/* FP:config.rs-3435 */             Hash::hash(&key.len(), hasher);
/* FP:config.rs-3436 */             Hash::hash(key, hasher);
/* FP:config.rs-3437 */             sub_hash.hash(hasher, error_format, for_crate_hash);
/* FP:config.rs-3438 */         }
/* FP:config.rs-3439 */     }
/* FP:config.rs-3440 */ }
/* FP:config.rs-3441 */ 
/* FP:config.rs-3442 */ /// Default behavior to use in out-of-memory situations.
/* FP:config.rs-3443 */ #[derive(Clone, Copy, PartialEq, Hash, Debug, Encodable, Decodable, HashStable_Generic)]
/* FP:config.rs-3444 */ pub enum OomStrategy {
/* FP:config.rs-3445 */     /// Generate a panic that can be caught by `catch_unwind`.
/* FP:config.rs-3446 */     Panic,
/* FP:config.rs-3447 */ 
/* FP:config.rs-3448 */     /// Abort the process immediately.
/* FP:config.rs-3449 */     Abort,
/* FP:config.rs-3450 */ }
/* FP:config.rs-3451 */ 
/* FP:config.rs-3452 */ impl OomStrategy {
/* FP:config.rs-3453 */     pub const SYMBOL: &'static str = "__rust_alloc_error_handler_should_panic_v2";
/* FP:config.rs-3454 */ 
/* FP:config.rs-3455 */     pub fn should_panic(self) -> u8 {
/* FP:config.rs-3456 */         match self {
/* FP:config.rs-3457 */             OomStrategy::Panic => 1,
/* FP:config.rs-3458 */             OomStrategy::Abort => 0,
/* FP:config.rs-3459 */         }
/* FP:config.rs-3460 */     }
/* FP:config.rs-3461 */ }
/* FP:config.rs-3462 */ 
/* FP:config.rs-3463 */ /// How to run proc-macro code when building this crate
/* FP:config.rs-3464 */ #[derive(Clone, Copy, PartialEq, Hash, Debug)]
/* FP:config.rs-3465 */ pub enum ProcMacroExecutionStrategy {
/* FP:config.rs-3466 */     /// Run the proc-macro code on the same thread as the server.
/* FP:config.rs-3467 */     SameThread,
/* FP:config.rs-3468 */ 
/* FP:config.rs-3469 */     /// Run the proc-macro code on a different thread.
/* FP:config.rs-3470 */     CrossThread,
/* FP:config.rs-3471 */ }
/* FP:config.rs-3472 */ 
/* FP:config.rs-3473 */ /// How to perform collapse macros debug info
/* FP:config.rs-3474 */ /// if-ext - if macro from different crate (related to callsite code)
/* FP:config.rs-3475 */ /// | cmd \ attr    | no  | (unspecified) | external | yes |
/* FP:config.rs-3476 */ /// | no            | no  | no            | no       | no  |
/* FP:config.rs-3477 */ /// | (unspecified) | no  | no            | if-ext   | yes |
/* FP:config.rs-3478 */ /// | external      | no  | if-ext        | if-ext   | yes |
/* FP:config.rs-3479 */ /// | yes           | yes | yes           | yes      | yes |
/* FP:config.rs-3480 */ #[derive(Clone, Copy, PartialEq, Hash, Debug)]
/* FP:config.rs-3481 */ pub enum CollapseMacroDebuginfo {
/* FP:config.rs-3482 */     /// Don't collapse debuginfo for the macro
/* FP:config.rs-3483 */     No = 0,
/* FP:config.rs-3484 */     /// Unspecified value
/* FP:config.rs-3485 */     Unspecified = 1,
/* FP:config.rs-3486 */     /// Collapse debuginfo if the macro comes from a different crate
/* FP:config.rs-3487 */     External = 2,
/* FP:config.rs-3488 */     /// Collapse debuginfo for the macro
/* FP:config.rs-3489 */     Yes = 3,
/* FP:config.rs-3490 */ }
/* FP:config.rs-3491 */ 
/* FP:config.rs-3492 */ /// Which format to use for `-Z dump-mono-stats`
/* FP:config.rs-3493 */ #[derive(Clone, Copy, PartialEq, Hash, Debug)]
/* FP:config.rs-3494 */ pub enum DumpMonoStatsFormat {
/* FP:config.rs-3495 */     /// Pretty-print a markdown table
/* FP:config.rs-3496 */     Markdown,
/* FP:config.rs-3497 */     /// Emit structured JSON
/* FP:config.rs-3498 */     Json,
/* FP:config.rs-3499 */ }
/* FP:config.rs-3500 */ 
/* FP:config.rs-3501 */ impl DumpMonoStatsFormat {
/* FP:config.rs-3502 */     pub fn extension(self) -> &'static str {
/* FP:config.rs-3503 */         match self {
/* FP:config.rs-3504 */             Self::Markdown => "md",
/* FP:config.rs-3505 */             Self::Json => "json",
/* FP:config.rs-3506 */         }
/* FP:config.rs-3507 */     }
/* FP:config.rs-3508 */ }
/* FP:config.rs-3509 */ 
/* FP:config.rs-3510 */ /// `-Z patchable-function-entry` representation - how many nops to put before and after function
/* FP:config.rs-3511 */ /// entry.
/* FP:config.rs-3512 */ #[derive(Clone, Copy, PartialEq, Hash, Debug, Default)]
/* FP:config.rs-3513 */ pub struct PatchableFunctionEntry {
/* FP:config.rs-3514 */     /// Nops before the entry
/* FP:config.rs-3515 */     prefix: u8,
/* FP:config.rs-3516 */     /// Nops after the entry
/* FP:config.rs-3517 */     entry: u8,
/* FP:config.rs-3518 */ }
/* FP:config.rs-3519 */ 
/* FP:config.rs-3520 */ impl PatchableFunctionEntry {
/* FP:config.rs-3521 */     pub fn from_total_and_prefix_nops(
/* FP:config.rs-3522 */         total_nops: u8,
/* FP:config.rs-3523 */         prefix_nops: u8,
/* FP:config.rs-3524 */     ) -> Option<PatchableFunctionEntry> {
/* FP:config.rs-3525 */         if total_nops < prefix_nops {
/* FP:config.rs-3526 */             None
/* FP:config.rs-3527 */         } else {
/* FP:config.rs-3528 */             Some(Self { prefix: prefix_nops, entry: total_nops - prefix_nops })
/* FP:config.rs-3529 */         }
/* FP:config.rs-3530 */     }
/* FP:config.rs-3531 */     pub fn prefix(&self) -> u8 {
/* FP:config.rs-3532 */         self.prefix
/* FP:config.rs-3533 */     }
/* FP:config.rs-3534 */     pub fn entry(&self) -> u8 {
/* FP:config.rs-3535 */         self.entry
/* FP:config.rs-3536 */     }
/* FP:config.rs-3537 */ }
/* FP:config.rs-3538 */ 
/* FP:config.rs-3539 */ /// `-Zpolonius` values, enabling the borrow checker polonius analysis, and which version: legacy,
/* FP:config.rs-3540 */ /// or future prototype.
/* FP:config.rs-3541 */ #[derive(Clone, Copy, PartialEq, Hash, Debug, Default)]
/* FP:config.rs-3542 */ pub enum Polonius {
/* FP:config.rs-3543 */     /// The default value: disabled.
/* FP:config.rs-3544 */     #[default]
/* FP:config.rs-3545 */     Off,
/* FP:config.rs-3546 */ 
/* FP:config.rs-3547 */     /// Legacy version, using datalog and the `polonius-engine` crate. Historical value for `-Zpolonius`.
/* FP:config.rs-3548 */     Legacy,
/* FP:config.rs-3549 */ 
/* FP:config.rs-3550 */     /// In-tree prototype, extending the NLL infrastructure.
/* FP:config.rs-3551 */     Next,
/* FP:config.rs-3552 */ }
/* FP:config.rs-3553 */ 
/* FP:config.rs-3554 */ impl Polonius {
/* FP:config.rs-3555 */     /// Returns whether the legacy version of polonius is enabled
/* FP:config.rs-3556 */     pub fn is_legacy_enabled(&self) -> bool {
/* FP:config.rs-3557 */         matches!(self, Polonius::Legacy)
/* FP:config.rs-3558 */     }
/* FP:config.rs-3559 */ 
/* FP:config.rs-3560 */     /// Returns whether the "next" version of polonius is enabled
/* FP:config.rs-3561 */     pub fn is_next_enabled(&self) -> bool {
/* FP:config.rs-3562 */         matches!(self, Polonius::Next)
/* FP:config.rs-3563 */     }
/* FP:config.rs-3564 */ }
/* FP:config.rs-3565 */ 
/* FP:config.rs-3566 */ #[derive(Clone, Copy, PartialEq, Hash, Debug)]
/* FP:config.rs-3567 */ pub enum InliningThreshold {
/* FP:config.rs-3568 */     Always,
/* FP:config.rs-3569 */     Sometimes(usize),
/* FP:config.rs-3570 */     Never,
/* FP:config.rs-3571 */ }
/* FP:config.rs-3572 */ 
/* FP:config.rs-3573 */ impl Default for InliningThreshold {
/* FP:config.rs-3574 */     fn default() -> Self {
/* FP:config.rs-3575 */         Self::Sometimes(100)
/* FP:config.rs-3576 */     }
/* FP:config.rs-3577 */ }
/* FP:config.rs-3578 */ 
/* FP:config.rs-3579 */ /// The different settings that the `-Zfunction-return` flag can have.
/* FP:config.rs-3580 */ #[derive(Clone, Copy, PartialEq, Hash, Debug, Default)]
/* FP:config.rs-3581 */ pub enum FunctionReturn {
/* FP:config.rs-3582 */     /// Keep the function return unmodified.
/* FP:config.rs-3583 */     #[default]
/* FP:config.rs-3584 */     Keep,
/* FP:config.rs-3585 */ 
/* FP:config.rs-3586 */     /// Replace returns with jumps to thunk, without emitting the thunk.
/* FP:config.rs-3587 */     ThunkExtern,
/* FP:config.rs-3588 */ }
/* FP:config.rs-3589 */ 
/* FP:config.rs-3590 */ /// Whether extra span comments are included when dumping MIR, via the `-Z mir-include-spans` flag.
/* FP:config.rs-3591 */ /// By default, only enabled in the NLL MIR dumps, and disabled in all other passes.
/* FP:config.rs-3592 */ #[derive(Clone, Copy, Default, PartialEq, Debug)]
/* FP:config.rs-3593 */ pub enum MirIncludeSpans {
/* FP:config.rs-3594 */     Off,
/* FP:config.rs-3595 */     On,
/* FP:config.rs-3596 */     /// Default: include extra comments in NLL MIR dumps only. Can be ignored and considered as
/* FP:config.rs-3597 */     /// `Off` in all other cases.
/* FP:config.rs-3598 */     #[default]
/* FP:config.rs-3599 */     Nll,
/* FP:config.rs-3600 */ }
/* FP:config.rs-3601 */ 
/* FP:config.rs-3602 */ impl MirIncludeSpans {
/* FP:config.rs-3603 */     /// Unless opting into extra comments for all passes, they can be considered disabled.
/* FP:config.rs-3604 */     /// The cases where a distinction between on/off and a per-pass value can exist will be handled
/* FP:config.rs-3605 */     /// in the passes themselves: i.e. the `Nll` value is considered off for all intents and
/* FP:config.rs-3606 */     /// purposes, except for the NLL MIR dump pass.
/* FP:config.rs-3607 */     pub fn is_enabled(self) -> bool {
/* FP:config.rs-3608 */         self == MirIncludeSpans::On
/* FP:config.rs-3609 */     }
/* FP:config.rs-3610 */ }