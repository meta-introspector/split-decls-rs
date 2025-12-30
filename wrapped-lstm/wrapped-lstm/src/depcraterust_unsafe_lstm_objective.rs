// Generated macro for rust_unsafe_lstm_objective (function)
macro_rules! Depcraterust_unsafe_lstm_objective {
() => {
// Module: crate
// Provides: {"rust_unsafe_lstm_objective"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn rust_unsafe_lstm_objective (l : i32 , c : i32 , b : i32 , main_params : * const f64 , extra_params : * const f64 , state : * mut f64 , sequence : * const f64 , loss : * mut f64) { let l = l as usize ; let c = c as usize ; let b = b as usize ; unsafe { unsf :: lstm_unsafe_objective (l , c , b , main_params , extra_params , state , sequence , loss) ; } }
};
}
