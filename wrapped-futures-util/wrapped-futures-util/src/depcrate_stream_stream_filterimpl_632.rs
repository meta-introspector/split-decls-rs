// Generated macro for impl_632 (impl)
macro_rules! Depcrate_stream_stream_filterimpl_632 {
() => {
// Module: crate::stream::stream::filter
// Provides: {"impl_632"}
// Dependencies: {}
impl < St , Fut , F > Filter < St , Fut , F > where St : Stream , F : for < 'a > FnMut1 < & 'a St :: Item , Output = Fut > , Fut : Future < Output = bool > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , pending_fut : None , pending_item : None } } delegate_access_inner ! (stream , St , ()) ; }
};
}
