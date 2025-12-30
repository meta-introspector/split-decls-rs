// Generated macro for regression (function)
macro_rules! Depcrate_testsregression {
() => {
// Module: crate::tests
// Provides: {"regression"}
// Dependencies: {}
# [test] fn regression () { check_dnf ("#![cfg(all(not(not(any(any(any()))))))]" , expect ! [[r##"#![cfg(any())]"##]]) ; check_dnf ("#![cfg(all(any(all(any()))))]" , expect ! [[r##"#![cfg(any())]"##]]) ; check_dnf ("#![cfg(all(all(any())))]" , expect ! [[r##"#![cfg(any())]"##]]) ; check_dnf ("#![cfg(all(all(any(), x)))]" , expect ! [[r##"#![cfg(any())]"##]]) ; check_dnf ("#![cfg(all(all(any()), x))]" , expect ! [[r##"#![cfg(any())]"##]]) ; check_dnf ("#![cfg(all(all(any(x))))]" , expect ! [[r##"#![cfg(x)]"##]]) ; check_dnf ("#![cfg(all(all(any(x), x)))]" , expect ! [[r##"#![cfg(all(x, x))]"##]]) ; }
};
}
