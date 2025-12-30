// Generated macro for rust_safe_lstm_objective (function)
macro_rules! Depcraterust_safe_lstm_objective {
() => {
// Module: crate
// Provides: {"rust_safe_lstm_objective"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn rust_safe_lstm_objective (l : i32 , c : i32 , b : i32 , main_params : * const f64 , extra_params : * const f64 , state : * mut f64 , sequence : * const f64 , loss : * mut f64) { let l = l as usize ; let c = c as usize ; let b = b as usize ; let (main_params , extra_params , state , sequence) = unsafe { (slice :: from_raw_parts (main_params , 2 * l * 4 * b) , slice :: from_raw_parts (extra_params , 3 * b) , slice :: from_raw_parts_mut (state , 2 * l * b) , slice :: from_raw_parts (sequence , c * b)) } ; unsafe { safe :: lstm_objective (l , c , b , main_params , extra_params , state , sequence , & mut * loss) ; } }
};
}
