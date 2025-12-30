// Generated macro for test_group (function)
macro_rules! Depcrate_rendertest_group {
() => {
// Module: crate::render
// Provides: {"test_group"}
// Dependencies: {}
fn test_group (mut test : ItemFn , rendered_cases : TokenStream) -> TokenStream { let fname = & test . sig . ident ; test . attrs = vec ! [] ; quote ! { # [cfg (test)] # test # [cfg (test)] mod # fname { use super ::*; # rendered_cases } } }
};
}
