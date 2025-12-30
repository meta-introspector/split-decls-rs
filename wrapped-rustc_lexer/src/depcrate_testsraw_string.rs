// Generated macro for raw_string (function)
macro_rules! Depcrate_testsraw_string {
() => {
// Module: crate::tests
// Provides: {"raw_string"}
// Dependencies: {}
# [test] fn raw_string () { check_lexing ("r###\"\"#a\\b\x00c\"\"###" , FrontmatterAllowed :: No , expect ! [[r#"
            Token { kind: Literal { kind: RawStr { n_hashes: Some(3) }, suffix_start: 17 }, len: 17 }
        "#]] ,) }
};
}
