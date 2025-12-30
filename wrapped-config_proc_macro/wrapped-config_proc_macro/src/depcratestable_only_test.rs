// Generated macro for stable_only_test (function)
macro_rules! Depcratestable_only_test {
() => {
// Module: crate
// Provides: {"stable_only_test"}
// Dependencies: {}
# [doc = " Used to conditionally output the TokenStream for tests that need to be run on stable only."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use rustfmt_config_proc_macro::stable_only_test;"] # [doc = ""] # [doc = " #[stable_only_test]"] # [doc = " #[test]"] # [doc = " fn test_needs_stable_rustfmt() {"] # [doc = "   assert!(true);"] # [doc = " }"] # [doc = " ```"] # [proc_macro_attribute] pub fn stable_only_test (_args : TokenStream , input : TokenStream) -> TokenStream { if option_env ! ("CFG_RELEASE_CHANNEL") . map_or (false , | c | c == "stable") { input } else { TokenStream :: from_str ("") . unwrap () } }
};
}
