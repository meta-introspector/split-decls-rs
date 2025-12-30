// Generated macro for buffer (module)
macro_rules! Depcrate_ffi_utilsbuffer {
() => {
// Module: crate::ffi_utils
// Provides: {"buffer"}
// Dependencies: {}
# [doc = " A buffer to store the timezone name when calling the C API."] # [cfg (any (test , target_vendor = "apple" , target_env = "ohos"))] pub (crate) mod buffer { # [doc = " The longest name in the IANA time zone database is 32 ASCII characters long."] pub const MAX_LEN : usize = 64 ; # [doc = " Return a buffer to store the timezone name."] # [doc = ""] # [doc = " The buffer is used to store the timezone name when calling the C API."] pub const fn tzname_buf () -> [u8 ; MAX_LEN] { [0 ; MAX_LEN] } }
};
}
