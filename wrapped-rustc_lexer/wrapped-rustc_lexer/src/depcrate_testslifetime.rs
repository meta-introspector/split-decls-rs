// Generated macro for lifetime (function)
macro_rules! Depcrate_testslifetime {
() => {
// Module: crate::tests
// Provides: {"lifetime"}
// Dependencies: {}
# [test] fn lifetime () { check_lexing ("'abc" , FrontmatterAllowed :: No , expect ! [[r#"
            Token { kind: Lifetime { starts_with_number: false }, len: 4 }
        "#]] ,) ; }
};
}
