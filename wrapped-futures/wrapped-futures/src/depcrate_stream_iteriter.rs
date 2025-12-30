// Generated macro for iter (function)
macro_rules! Depcrate_stream_iteriter {
() => {
// Module: crate::stream::iter
// Provides: {"iter"}
// Dependencies: {}
# [doc = " Converts an `Iterator` into a `Stream` which is always ready to yield the"] # [doc = " next value."] # [doc = ""] # [doc = " Iterators in Rust don't express the ability to block, so this adapter simply"] # [doc = " always calls `iter.next()` and returns that. Additionally, the error type is"] # [doc = " generic here as it will never be returned, instead the type of the iterator"] # [doc = " will always be returned upwards as a successful value."] pub fn iter < I , T , E > (i : I) -> IterStream < I > where I : Iterator < Item = Result < T , E > > , I : Send + 'static , T : Send + 'static , E : Send + 'static , { IterStream { iter : i , } }
};
}
