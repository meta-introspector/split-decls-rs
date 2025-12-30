// Generated macro for set_return_data (function)
macro_rules! Depcrateset_return_data {
() => {
// Module: crate
// Provides: {"set_return_data"}
// Dependencies: {}
# [doc = " Set the running program's return data."] # [doc = ""] # [doc = " Return data is a dedicated per-transaction buffer for data passed"] # [doc = " from cross-program invoked programs back to their caller."] # [doc = ""] # [doc = " The maximum size of return data is [`MAX_RETURN_DATA`]. Return data is"] # [doc = " retrieved by the caller with [`get_return_data`]."] # [allow (unused_variables)] pub fn set_return_data (data : & [u8]) { # [cfg (target_os = "solana")] unsafe { crate :: syscalls :: sol_set_return_data (data . as_ptr () , data . len () as u64) } ; }
};
}
