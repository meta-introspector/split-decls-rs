// Generated macro for rust_lorenz_unsf (function)
macro_rules! Depcraterust_lorenz_unsf {
() => {
// Module: crate
// Provides: {"rust_lorenz_unsf"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn rust_lorenz_unsf (x : * const StateType , dxdt : * mut StateType , t : f64) { let x : & StateType = unsafe { & * x } ; let dxdt : & mut StateType = unsafe { & mut * dxdt } ; unsafe { unsf :: lorenz (x , dxdt , t) } ; }
};
}
