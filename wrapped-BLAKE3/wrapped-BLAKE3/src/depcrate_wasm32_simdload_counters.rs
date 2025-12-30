// Generated macro for load_counters (function)
macro_rules! Depcrate_wasm32_simdload_counters {
() => {
// Module: crate::wasm32_simd
// Provides: {"load_counters"}
// Dependencies: {}
# [inline (always)] fn load_counters (counter : u64 , increment_counter : IncrementCounter) -> (v128 , v128) { let mask = if increment_counter . yes () { ! 0 } else { 0 } ; (set4 (counter_low (counter + (mask & 0)) , counter_low (counter + (mask & 1)) , counter_low (counter + (mask & 2)) , counter_low (counter + (mask & 3)) ,) , set4 (counter_high (counter + (mask & 0)) , counter_high (counter + (mask & 1)) , counter_high (counter + (mask & 2)) , counter_high (counter + (mask & 3)) ,) ,) }
};
}
