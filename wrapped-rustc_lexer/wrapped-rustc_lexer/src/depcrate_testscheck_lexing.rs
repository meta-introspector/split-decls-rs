// Generated macro for check_lexing (function)
macro_rules! Depcrate_testscheck_lexing {
() => {
// Module: crate::tests
// Provides: {"check_lexing"}
// Dependencies: {}
fn check_lexing (src : & str , frontmatter_allowed : FrontmatterAllowed , expect : Expect) { let actual : String = tokenize (src , frontmatter_allowed) . map (| token | format ! ("{:?}\n" , token)) . collect () ; expect . assert_eq (& actual) }
};
}
