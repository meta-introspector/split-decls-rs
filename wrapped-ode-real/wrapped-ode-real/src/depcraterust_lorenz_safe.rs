// Generated macro for rust_lorenz_safe (function)
macro_rules! Depcraterust_lorenz_safe {
() => {
// Module: crate
// Provides: {"rust_lorenz_safe"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn rust_lorenz_safe (x : * const StateType , dxdt : * mut StateType , t : f64) { let x : & StateType = unsafe { & * x } ; let dxdt : & mut StateType = unsafe { & mut * dxdt } ; safe :: lorenz (x , dxdt , t) ; }
};
}
