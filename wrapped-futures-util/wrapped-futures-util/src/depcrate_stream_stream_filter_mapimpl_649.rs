// Generated macro for impl_649 (impl)
macro_rules! Depcrate_stream_stream_filter_mapimpl_649 {
() => {
// Module: crate::stream::stream::filter_map
// Provides: {"impl_649"}
// Dependencies: {}
impl < St , Fut , F > FilterMap < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , pending : None } } delegate_access_inner ! (stream , St , ()) ; }
};
}
