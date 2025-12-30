// Generated macro for impl_1089 (impl)
macro_rules! Depcrate_stream_stream_flatten_unorderedimpl_1089 {
() => {
// Module: crate::stream::stream::flatten_unordered
// Provides: {"impl_1089"}
// Dependencies: {}
impl < St , Fc > FusedStream for FlattenUnorderedWithFlowController < St , Fc > where St : FusedStream , Fc : FlowController < St :: Item , < St :: Item as Stream > :: Item > , St :: Item : Stream + Unpin , { fn is_terminated (& self) -> bool { self . stream . is_terminated () && self . inner_streams . is_empty () } }
};
}
