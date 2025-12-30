// Generated macro for Fold (struct)
macro_rules! Depcrate_stream_foldFold {
() => {
// Module: crate::stream::fold
// Provides: {"Fold"}
// Dependencies: {}
# [doc = " A future used to collect all the results of a stream into one generic type."] # [doc = ""] # [doc = " This future is returned by the `Stream::fold` method."] pub struct Fold < S , F , Fut , T > where Fut : IntoFuture { stream : S , f : F , state : State < T , Fut :: Future > , }
};
}
