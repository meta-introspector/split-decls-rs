// Generated macro for Stream (trait)
macro_rules! Depcrate_streamStream {
() => {
// Module: crate::stream
// Provides: {"Stream"}
// Dependencies: {}
# [doc = " A stream of tokens which can be duplicated"] # [doc = ""] # [doc = " This is a trait over types which implement the `StreamOnce`, `ResetStream` and `Positioned`"] # [doc = " traits. If you need a custom `Stream` object then implement those traits and `Stream` is"] # [doc = " implemented automatically."] pub trait Stream : StreamOnce + ResetStream + Positioned { }
};
}
