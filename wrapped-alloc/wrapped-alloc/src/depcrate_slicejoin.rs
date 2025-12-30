// Generated macro for Join (trait)
macro_rules! Depcrate_sliceJoin {
() => {
// Module: crate::slice
// Provides: {"Join"}
// Dependencies: {}
# [doc = " Helper trait for [`[T]::join`](slice::join)"] # [unstable (feature = "slice_concat_trait" , issue = "27747")] pub trait Join < Separator > { # [unstable (feature = "slice_concat_trait" , issue = "27747")] # [doc = " The resulting type after concatenation"] type Output ; # [doc = " Implementation of [`[T]::join`](slice::join)"] # [unstable (feature = "slice_concat_trait" , issue = "27747")] fn join (slice : & Self , sep : Separator) -> Self :: Output ; }
};
}
