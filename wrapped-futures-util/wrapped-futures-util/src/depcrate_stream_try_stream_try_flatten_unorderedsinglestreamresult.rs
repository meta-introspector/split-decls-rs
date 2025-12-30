// Generated macro for SingleStreamResult (type)
macro_rules! Depcrate_stream_try_stream_try_flatten_unorderedSingleStreamResult {
() => {
// Module: crate::stream::try_stream::try_flatten_unordered
// Provides: {"SingleStreamResult"}
// Dependencies: {}
type SingleStreamResult < St > = Single < Result < < St as TryStream > :: Ok , < St as TryStream > :: Error > > ;
};
}
