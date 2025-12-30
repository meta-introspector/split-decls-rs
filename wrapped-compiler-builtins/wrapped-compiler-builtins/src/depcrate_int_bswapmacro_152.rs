// Generated macro for macro_152 (macro)
macro_rules! Depcrate_int_bswapmacro_152 {
() => {
// Module: crate::int::bswap
// Provides: {"macro_152"}
// Dependencies: {}
intrinsics ! { # [maybe_use_optimized_c_shim] # [doc = " Swaps bytes in 32-bit number"] pub extern "C" fn __bswapsi2 (x : u32) -> u32 { x . swap_bytes () } # [maybe_use_optimized_c_shim] # [doc = " Swaps bytes in 64-bit number"] pub extern "C" fn __bswapdi2 (x : u64) -> u64 { x . swap_bytes () } # [maybe_use_optimized_c_shim] # [doc = " Swaps bytes in 128-bit number"] pub extern "C" fn __bswapti2 (x : u128) -> u128 { x . swap_bytes () } }
};
}
