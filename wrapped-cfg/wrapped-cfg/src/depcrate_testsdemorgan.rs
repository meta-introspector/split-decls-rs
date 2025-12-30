// Generated macro for demorgan (function)
macro_rules! Depcrate_testsdemorgan {
() => {
// Module: crate::tests
// Provides: {"demorgan"}
// Dependencies: {}
# [test] fn demorgan () { check_dnf ("#![cfg(not(all(a, b)))]" , expect ! [[r#"#![cfg(any(not(a), not(b)))]"#]]) ; check_dnf ("#![cfg(not(any(a, b)))]" , expect ! [[r#"#![cfg(all(not(a), not(b)))]"#]]) ; check_dnf ("#![cfg(not(all(not(a), b)))]" , expect ! [[r#"#![cfg(any(a, not(b)))]"#]]) ; check_dnf ("#![cfg(not(any(a, not(b))))]" , expect ! [[r#"#![cfg(all(not(a), b))]"#]]) ; }
};
}
