/* FP:lib.rs-0001 */ // # Feature gates
/* FP:lib.rs-0002 */ //
/* FP:lib.rs-0003 */ // This crate declares the set of past and present unstable features in the compiler.
/* FP:lib.rs-0004 */ // Feature gate checking itself is done in `rustc_ast_passes/src/feature_gate.rs`
/* FP:lib.rs-0005 */ // at the moment.
/* FP:lib.rs-0006 */ //
/* FP:lib.rs-0007 */ // Features are enabled in programs via the crate-level attributes of
/* FP:lib.rs-0008 */ // `#[feature(...)]` with a comma-separated list of features.
/* FP:lib.rs-0009 */ //
/* FP:lib.rs-0010 */ // For the purpose of future feature-tracking, once a feature gate is added,
/* FP:lib.rs-0011 */ // even if it is stabilized or removed, *do not remove it*. Instead, move the
/* FP:lib.rs-0012 */ // symbol to the `accepted` or `removed` modules respectively.
/* FP:lib.rs-0013 */ 
/* FP:lib.rs-0014 */ // tidy-alphabetical-start
/* FP:lib.rs-0015 */ #[allow(internal_features)]
/* FP:lib.rs-0016 */ #[doc(rust_logo)]
/* FP:lib.rs-0017 */ #[feature(rustdoc_internals)]
/* FP:lib.rs-0018 */ // tidy-alphabetical-end
/* FP:lib.rs-0019 */ 
/* FP:lib.rs-0024 */ 
/* FP:lib.rs-0025 */ #[cfg(test)]
/* FP:lib.rs-0027 */ 
/* FP:lib.rs-0028 */ use std::num::NonZero;
/* FP:lib.rs-0029 */ 
/* FP:lib.rs-0030 */ use crate::rustc_complete::Symbol;
/* FP:lib.rs-0031 */ 
/* FP:lib.rs-0032 */ #[derive(Debug, Clone)]
/* FP:lib.rs-0033 */ pub struct Feature {
/* FP:lib.rs-0034 */     pub name: Symbol,
/* FP:lib.rs-0035 */     /// For unstable features: the version the feature was added in.
/* FP:lib.rs-0036 */     /// For accepted features: the version the feature got stabilized in.
/* FP:lib.rs-0037 */     /// For removed features we are inconsistent; sometimes this is the
/* FP:lib.rs-0038 */     /// version it got added, sometimes the version it got removed.
/* FP:lib.rs-0039 */     pub since: &'static str,
/* FP:lib.rs-0040 */     issue: Option<NonZero<u32>>,
/* FP:lib.rs-0041 */ }
/* FP:lib.rs-0042 */ 
/* FP:lib.rs-0043 */ #[derive(Clone, Copy, Debug, Hash)]
/* FP:lib.rs-0044 */ pub enum UnstableFeatures {
/* FP:lib.rs-0045 */     /// Disallow use of unstable features, as on beta/stable channels.
/* FP:lib.rs-0046 */     Disallow,
/* FP:lib.rs-0047 */     /// Allow use of unstable features, as on nightly.
/* FP:lib.rs-0048 */     Allow,
/* FP:lib.rs-0049 */     /// Errors are bypassed for bootstrapping. This is required any time
/* FP:lib.rs-0050 */     /// during the build that feature-related lints are set to warn or above
/* FP:lib.rs-0051 */     /// because the build turns on warnings-as-errors and uses lots of unstable
/* FP:lib.rs-0052 */     /// features. As a result, this is always required for building Rust itself.
/* FP:lib.rs-0053 */     Cheat,
/* FP:lib.rs-0054 */ }
/* FP:lib.rs-0055 */ 
/* FP:lib.rs-0056 */ impl UnstableFeatures {
/* FP:lib.rs-0057 */     /// This takes into account `RUSTC_BOOTSTRAP`.
/* FP:lib.rs-0058 */     ///
/* FP:lib.rs-0059 */     /// If `krate` is [`Some`], then setting `RUSTC_BOOTSTRAP=krate` will enable the nightly
/* FP:lib.rs-0060 */     /// features. Otherwise, only `RUSTC_BOOTSTRAP=1` will work.
/* FP:lib.rs-0061 */     pub fn from_environment(krate: Option<&str>) -> Self {
/* FP:lib.rs-0062 */         Self::from_environment_value(krate, std::env::var("RUSTC_BOOTSTRAP"))
/* FP:lib.rs-0063 */     }
/* FP:lib.rs-0064 */ 
/* FP:lib.rs-0065 */     /// Avoid unsafe `std::env::set_var()` by allowing tests to inject
/* FP:lib.rs-0066 */     /// `std::env::var("RUSTC_BOOTSTRAP")` with the `env_var_rustc_bootstrap`
/* FP:lib.rs-0067 */     /// arg.
/* FP:lib.rs-0068 */     fn from_environment_value(
/* FP:lib.rs-0069 */         krate: Option<&str>,
/* FP:lib.rs-0070 */         env_var_rustc_bootstrap: Result<String, std::env::VarError>,
/* FP:lib.rs-0071 */     ) -> Self {
/* FP:lib.rs-0072 */         // `true` if this is a feature-staged build, i.e., on the beta or stable channel.
/* FP:lib.rs-0073 */         let disable_unstable_features =
/* FP:lib.rs-0074 */             option_env!("CFG_DISABLE_UNSTABLE_FEATURES").is_some_and(|s| s != "0");
/* FP:lib.rs-0075 */         // Returns whether `krate` should be counted as unstable
/* FP:lib.rs-0076 */         let is_unstable_crate =
/* FP:lib.rs-0077 */             |var: &str| krate.is_some_and(|name| var.split(',').any(|new_krate| new_krate == name));
/* FP:lib.rs-0078 */ 
/* FP:lib.rs-0079 */         let bootstrap = env_var_rustc_bootstrap.ok();
/* FP:lib.rs-0080 */         if let Some(val) = bootstrap.as_deref() {
/* FP:lib.rs-0081 */             match val {
/* FP:lib.rs-0082 */                 val if val == "1" || is_unstable_crate(val) => return UnstableFeatures::Cheat,
/* FP:lib.rs-0083 */                 // Hypnotize ourselves so that we think we are a stable compiler and thus don't
/* FP:lib.rs-0084 */                 // allow any unstable features.
/* FP:lib.rs-0085 */                 "-1" => return UnstableFeatures::Disallow,
/* FP:lib.rs-0086 */                 _ => {}
/* FP:lib.rs-0087 */             }
/* FP:lib.rs-0088 */         }
/* FP:lib.rs-0089 */ 
/* FP:lib.rs-0090 */         if disable_unstable_features { UnstableFeatures::Disallow } else { UnstableFeatures::Allow }
/* FP:lib.rs-0091 */     }
/* FP:lib.rs-0092 */ 
/* FP:lib.rs-0093 */     pub fn is_nightly_build(&self) -> bool {
/* FP:lib.rs-0094 */         match *self {
/* FP:lib.rs-0095 */             UnstableFeatures::Allow | UnstableFeatures::Cheat => true,
/* FP:lib.rs-0096 */             UnstableFeatures::Disallow => false,
/* FP:lib.rs-0097 */         }
/* FP:lib.rs-0098 */     }
/* FP:lib.rs-0099 */ }
/* FP:lib.rs-0100 */ 
/* FP:lib.rs-0101 */ fn find_lang_feature_issue(feature: Symbol) -> Option<NonZero<u32>> {
/* FP:lib.rs-0102 */     // Search in all the feature lists.
/* FP:lib.rs-0103 */     if let Some(f) = UNSTABLE_LANG_FEATURES.iter().find(|f| f.name == feature) {
/* FP:lib.rs-0104 */         return f.issue;
/* FP:lib.rs-0105 */     }
/* FP:lib.rs-0106 */     if let Some(f) = ACCEPTED_LANG_FEATURES.iter().find(|f| f.name == feature) {
/* FP:lib.rs-0107 */         return f.issue;
/* FP:lib.rs-0108 */     }
/* FP:lib.rs-0109 */     if let Some(f) = REMOVED_LANG_FEATURES.iter().find(|f| f.feature.name == feature) {
/* FP:lib.rs-0110 */         return f.feature.issue;
/* FP:lib.rs-0111 */     }
/* FP:lib.rs-0112 */     panic!("feature `{feature}` is not declared anywhere");
/* FP:lib.rs-0113 */ }
/* FP:lib.rs-0114 */ 
/* FP:lib.rs-0115 */ const fn to_nonzero(n: Option<u32>) -> Option<NonZero<u32>> {
/* FP:lib.rs-0116 */     // Can be replaced with `n.and_then(NonZero::new)` if that is ever usable
/* FP:lib.rs-0117 */     // in const context. Requires https://github.com/rust-lang/rfcs/pull/2632.
/* FP:lib.rs-0118 */     match n {
/* FP:lib.rs-0119 */         None => None,
/* FP:lib.rs-0120 */         Some(n) => NonZero::new(n),
/* FP:lib.rs-0121 */     }
/* FP:lib.rs-0122 */ }
/* FP:lib.rs-0123 */ 
/* FP:lib.rs-0124 */ pub enum GateIssue {
/* FP:lib.rs-0125 */     Language,
/* FP:lib.rs-0126 */     Library(Option<NonZero<u32>>),
/* FP:lib.rs-0127 */ }
/* FP:lib.rs-0128 */ 
/* FP:lib.rs-0129 */ pub fn find_feature_issue(feature: Symbol, issue: GateIssue) -> Option<NonZero<u32>> {
/* FP:lib.rs-0130 */     match issue {
/* FP:lib.rs-0131 */         GateIssue::Language => find_lang_feature_issue(feature),
/* FP:lib.rs-0132 */         GateIssue::Library(lib) => lib,
/* FP:lib.rs-0133 */     }
/* FP:lib.rs-0134 */ }
/* FP:lib.rs-0135 */ 
/* FP:lib.rs-0136 */ pub use accepted::ACCEPTED_LANG_FEATURES;
/* FP:lib.rs-0137 */ pub use builtin_attrs::{
/* FP:lib.rs-0138 */     AttributeDuplicates, AttributeGate, AttributeSafety, AttributeTemplate, AttributeType,
/* FP:lib.rs-0139 */     BUILTIN_ATTRIBUTE_MAP, BUILTIN_ATTRIBUTES, BuiltinAttribute, GatedCfg, encode_cross_crate,
/* FP:lib.rs-0140 */     find_gated_cfg, is_builtin_attr_name, is_stable_diagnostic_attribute, is_valid_for_get_attr,
/* FP:lib.rs-0141 */ };
/* FP:lib.rs-0142 */ pub use removed::REMOVED_LANG_FEATURES;
/* FP:lib.rs-0143 */ pub use unstable::{
/* FP:lib.rs-0144 */     EnabledLangFeature, EnabledLibFeature, Features, INCOMPATIBLE_FEATURES, UNSTABLE_LANG_FEATURES,
/* FP:lib.rs-0145 */ };