// Generated macro for peek_nth (function)
macro_rules! Depcrate_peek_nthpeek_nth {
() => {
// Module: crate::peek_nth
// Provides: {"peek_nth"}
// Dependencies: {}
# [doc = " A drop-in replacement for [`std::iter::Peekable`] which adds a `peek_nth`"] # [doc = " method allowing the user to `peek` at a value several iterations forward"] # [doc = " without advancing the base iterator."] # [doc = ""] # [doc = " This differs from `multipeek` in that subsequent calls to `peek` or"] # [doc = " `peek_nth` will always return the same value until `next` is called"] # [doc = " (making `reset_peek` unnecessary)."] pub fn peek_nth < I > (iterable : I) -> PeekNth < I :: IntoIter > where I : IntoIterator , { PeekNth { iter : iterable . into_iter () . fuse () , buf : VecDeque :: new () , } }
};
}
