// Generated macro for RawTask (struct)
macro_rules! Depcrate_rawRawTask {
() => {
// Module: crate::raw
// Provides: {"RawTask"}
// Dependencies: {}
# [doc = " Raw pointers to the fields inside a task."] pub (crate) struct RawTask < F , T , S , M > { # [doc = " The task header."] pub (crate) header : * const HeaderWithMetadata < M > , # [doc = " The schedule function."] pub (crate) schedule : * const S , # [doc = " The future."] pub (crate) future : * mut F , # [doc = " The output of the future."] pub (crate) output : * mut Result < T , Panic > , }
};
}
