// Generated macro for impl_1088 (impl)
macro_rules! Depcrate_stream_stream_flatten_unorderedimpl_1088 {
() => {
// Module: crate::stream::stream::flatten_unordered
// Provides: {"impl_1088"}
// Dependencies: {}
impl < St , Fc > FlattenUnorderedWithFlowControllerProj < '_ , St , Fc > where St : Stream , { # [doc = " Checks if current `inner_streams` bucket size is greater than optional limit."] fn is_exceeded_limit (& self) -> bool { self . limit . map_or (false , | limit | self . inner_streams . len () >= limit . get ()) } }
};
}
