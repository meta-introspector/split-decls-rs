// Generated macro for nightly_only_test (function)
macro_rules! Depcratenightly_only_test {
() => {
// Module: crate
// Provides: {"nightly_only_test"}
// Dependencies: {}
# [doc = " Used to conditionally output the TokenStream for tests that need to be run on nightly only."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use rustfmt_config_proc_macro::nightly_only_test;"] # [doc = ""] # [doc = " #[nightly_only_test]"] # [doc = " #[test]"] # [doc = " fn test_needs_nightly_rustfmt() {"] # [doc = "   assert!(true);"] # [doc = " }"] # [doc = " ```"] # [proc_macro_attribute] pub fn nightly_only_test (_args : TokenStream , input : TokenStream) -> TokenStream { if option_env ! ("CFG_RELEASE_CHANNEL") . map_or (true , | c | c == "nightly" || c == "dev") { input } else { TokenStream :: from_str ("") . unwrap () } }
};
}
