// Generated macro for c_try (function)
macro_rules! Depcrate_callc_try {
() => {
// Module: crate::call
// Provides: {"c_try"}
// Dependencies: {}
pub fn c_try (ret : libc :: c_int) -> Result < libc :: c_int , Error > { match ret { n if n < 0 => Err (last_error (n)) , n => Ok (n) , } }
};
}
