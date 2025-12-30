// Generated macro for rustfmt_only_ci_test (function)
macro_rules! Depcraterustfmt_only_ci_test {
() => {
// Module: crate
// Provides: {"rustfmt_only_ci_test"}
// Dependencies: {}
# [doc = " Used to conditionally output the TokenStream for tests that should be run as part of rustfmts"] # [doc = " test suite, but should be ignored when running in the rust-lang/rust test suite."] # [proc_macro_attribute] pub fn rustfmt_only_ci_test (_args : TokenStream , input : TokenStream) -> TokenStream { if option_env ! ("RUSTFMT_CI") . is_some () { input } else { let mut token_stream = TokenStream :: from_str ("#[ignore]") . unwrap () ; token_stream . extend (input) ; token_stream } }
};
}
