// Generated macro for rust_unsafe_dlstm_objective (function)
macro_rules! Depcraterust_unsafe_dlstm_objective {
() => {
// Module: crate
// Provides: {"rust_unsafe_dlstm_objective"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn rust_unsafe_dlstm_objective (l : i32 , c : i32 , b : i32 , main_params : * const f64 , d_main_params : * mut f64 , extra_params : * const f64 , d_extra_params : * mut f64 , state : * mut f64 , sequence : * const f64 , res : * mut f64 , d_res : * mut f64) { let l = l as usize ; let c = c as usize ; let b = b as usize ; unsafe { unsf :: d_lstm_unsafe_objective (l , c , b , main_params , d_main_params , extra_params , d_extra_params , state , sequence , res , d_res) ; } }
};
}
