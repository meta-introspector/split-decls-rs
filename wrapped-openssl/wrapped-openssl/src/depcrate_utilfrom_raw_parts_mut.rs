// Generated macro for from_raw_parts_mut (function)
macro_rules! Depcrate_utilfrom_raw_parts_mut {
() => {
// Module: crate::util
// Provides: {"from_raw_parts_mut"}
// Dependencies: {}
# [doc = " The same as `slice::from_raw_parts_mut`, except that `data` may be `NULL`"] # [doc = " if `len` is 0."] pub unsafe fn from_raw_parts_mut < 'a , T > (data : * mut T , len : usize) -> & 'a mut [T] { if len == 0 { & mut [] } else { # [allow (clippy :: disallowed_methods)] slice :: from_raw_parts_mut (data , len) } }
};
}
