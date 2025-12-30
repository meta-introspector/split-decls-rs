// Generated macro for test (function)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [deprecated (since = "0.2.3" , note = "this crate is deprecated in favor of module-level #[coverage(off)] attribute; \
            see <https://github.com/taiki-e/cargo-llvm-cov?tab=readme-ov-file#exclude-code-from-coverage> \
            for more")] # [proc_macro_attribute] pub fn test (args : TokenStream , input : TokenStream) -> TokenStream { if ! args . is_empty () { return format_err ! (Span :: call_site () , "attribute must be of the form `#[test]`") . into_compile_error () ; } let mut out = TokenStream :: new () ; if cfg ! (coverage_helper_has_coverage_attribute) { out . extend (quote ! { # [cfg_attr (all (coverage_nightly , test) , coverage (off))] }) ; } out . extend (quote ! { # [:: core :: prelude :: v1 :: test] }) ; out . extend (input) ; out }
};
}
