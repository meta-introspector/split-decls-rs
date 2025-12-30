// Generated macro for BufSplit (trait)
macro_rules! Depcrate_range_bufBufSplit {
() => {
// Module: crate::range_buf
// Provides: {"BufSplit"}
// Dependencies: {}
# [doc = " A trait that enables zero-copy sends to quiche. When buffers produced"] # [doc = " by the `BufFactory` implement this trait, quiche and h3 can supply the"] # [doc = " raw buffers to be sent, instead of slices that must be copied first."] pub trait BufSplit { # [doc = " Split the buffer at a given point, after the split the old buffer"] # [doc = " must only contain the first `at` bytes, while the newly produced"] # [doc = " buffer must containt the remaining bytes."] fn split_at (& mut self , at : usize) -> Self ; # [doc = " Try to prepend a prefix to the buffer, return true if succeeded."] fn try_add_prefix (& mut self , _prefix : & [u8]) -> bool { false } }
};
}
