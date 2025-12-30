// Generated macro for is_prime (function)
macro_rules! Depcrate_testsis_prime {
() => {
// Module: crate::tests
// Provides: {"is_prime"}
// Dependencies: {}
fn is_prime (n : usize) -> bool { n != 0 && n != 1 && (2 ..) . take_while (| i | i * i <= n) . all (| i | n % i != 0) }
};
}
