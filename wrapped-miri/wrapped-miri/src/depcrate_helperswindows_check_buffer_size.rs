// Generated macro for windows_check_buffer_size (function)
macro_rules! Depcrate_helperswindows_check_buffer_size {
() => {
// Module: crate::helpers
// Provides: {"windows_check_buffer_size"}
// Dependencies: {}
# [doc = " Check whether an operation that writes to a target buffer was successful."] # [doc = " Accordingly select return value."] # [doc = " Local helper function to be used in Windows shims."] pub (crate) fn windows_check_buffer_size ((success , len) : (bool , u64)) -> u32 { if success { u32 :: try_from (len . strict_sub (1)) . unwrap () } else { u32 :: try_from (len) . unwrap () } }
};
}
