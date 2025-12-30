// Generated macro for c_result (function)
macro_rules! Depcrate_unix_termc_result {
() => {
// Module: crate::unix_term
// Provides: {"c_result"}
// Dependencies: {}
fn c_result < F : FnOnce () -> libc :: c_int > (f : F) -> io :: Result < () > { let res = f () ; if res != 0 { Err (io :: Error :: last_os_error ()) } else { Ok (()) } }
};
}
