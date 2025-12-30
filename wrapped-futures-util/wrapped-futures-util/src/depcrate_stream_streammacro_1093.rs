// Generated macro for macro_1093 (macro)
macro_rules! Depcrate_stream_streammacro_1093 {
() => {
// Module: crate::stream::stream
// Provides: {"macro_1093"}
// Dependencies: {}
# [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] # [cfg (feature = "alloc")] delegate_all ! (# [doc = " Stream for the [`flat_map_unordered`](StreamExt::flat_map_unordered) method."] FlatMapUnordered < St , U , F > (FlattenUnordered < Map < St , F >>) : Debug + Sink + Stream + FusedStream + AccessInner [St , (. .)] + New [| x : St , limit : Option < usize >, f : F | FlattenUnordered :: new (Map :: new (x , f) , limit)] where St : Stream , U : Stream , U : Unpin , F : FnMut (St :: Item) -> U) ;
};
}
