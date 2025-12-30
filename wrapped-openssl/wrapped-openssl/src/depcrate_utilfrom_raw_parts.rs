// Generated macro for from_raw_parts (function)
macro_rules! Depcrate_utilfrom_raw_parts {
() => {
// Module: crate::util
// Provides: {"from_raw_parts"}
// Dependencies: {}
# [doc = " The same as `slice::from_raw_parts`, except that `data` may be `NULL` if"] # [doc = " `len` is 0."] pub unsafe fn from_raw_parts < 'a , T > (data : * const T , len : usize) -> & 'a [T] { if len == 0 { & [] } else { # [allow (clippy :: disallowed_methods)] slice :: from_raw_parts (data , len) } }
};
}
