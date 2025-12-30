// Generated macro for JitterRng (struct)
macro_rules! DepcrateJitterRng {
() => {
// Module: crate
// Provides: {"JitterRng"}
// Dependencies: {}
# [doc = " A true random number generator based on jitter in the CPU execution time,"] # [doc = " and jitter in memory access time."] # [doc = ""] # [doc = " Note that this RNG is not suitable for use cases where cryptographic"] # [doc = " security is required."] pub struct JitterRng < F > { data : u64 , rounds : u8 , timer : F , mem_prev_index : u16 , data_half_used : bool , }
};
}
