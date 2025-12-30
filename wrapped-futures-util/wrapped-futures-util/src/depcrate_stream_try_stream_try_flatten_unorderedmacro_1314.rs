// Generated macro for macro_1314 (macro)
macro_rules! Depcrate_stream_try_stream_try_flatten_unorderedmacro_1314 {
() => {
// Module: crate::stream::try_stream::try_flatten_unordered
// Provides: {"macro_1314"}
// Dependencies: {}
pin_project ! { # [doc = " Emits either successful streams or single-item streams containing the underlying errors."] # [doc = " This's a wrapper for `FlattenUnordered` to reuse its logic over `TryStream`."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct NestedTryStreamIntoEitherTryStream < St > where St : TryStream , St :: Ok : TryStream , St :: Ok : Unpin , < St :: Ok as TryStream >:: Error : From < St :: Error > { # [pin] stream : St } }
};
}
