// Generated macro for set_return_data (function)
macro_rules! Depcrate_cpiset_return_data {
() => {
// Module: crate::cpi
// Provides: {"set_return_data"}
// Dependencies: {}
# [doc = " Set the running program's return data."] # [doc = ""] # [doc = " Return data is a dedicated per-transaction buffer for data passed"] # [doc = " from cross-program invoked programs back to their caller."] # [doc = ""] # [doc = " The maximum size of return data is [`MAX_RETURN_DATA`]. Return data is"] # [doc = " retrieved by the caller with [`get_return_data`]."] # [inline (always)] pub fn set_return_data (data : & [u8]) { # [cfg (any (target_os = "solana" , target_arch = "bpf"))] unsafe { sol_set_return_data (data . as_ptr () , data . len () as u64) } ; # [cfg (not (any (target_os = "solana" , target_arch = "bpf")))] core :: hint :: black_box (data) ; }
};
}
