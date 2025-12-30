// Generated macro for AsCCharPtr (trait)
macro_rules! Depcrate_commonAsCCharPtr {
() => {
// Module: crate::common
// Provides: {"AsCCharPtr"}
// Dependencies: {}
# [doc = " Extension trait for explicit casts to `*const c_char`."] pub (crate) trait AsCCharPtr { # [doc = " Equivalent to `self.as_ptr().cast()`, but only casts to `*const c_char`."] fn as_c_char_ptr (& self) -> * const c_char ; }
};
}
