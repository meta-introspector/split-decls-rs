// Generated macro for impl_230 (impl)
macro_rules! Depcrate_topo_layoutimpl_230 {
() => {
// Module: crate::topo::layout
// Provides: {"impl_230"}
// Dependencies: {}
impl VisualGraph { fn render (& self , debug : bool , rb : & mut dyn RenderBackend) { for node in & self . nodes { node . render (debug , rb) ; } for arrow in & self . edges { let mut elements = Vec :: new () ; for h in & arrow . 1 { elements . push (self . nodes [h . get_index ()] . clone ()) ; } render_arrow (rb , debug , & elements [..] , & arrow . 0) ; } } }
};
}
