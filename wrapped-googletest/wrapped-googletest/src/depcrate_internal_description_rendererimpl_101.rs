// Generated macro for impl_101 (impl)
macro_rules! Depcrate_internal_description_rendererimpl_101 {
() => {
// Module: crate::internal::description_renderer
// Provides: {"impl_101"}
// Dependencies: {}
impl From < & 'static str > for Block { fn from (value : & 'static str) -> Self { Block :: Literal (value . lines () . map (| v | Fragment (v . into ())) . collect ()) } }
};
}
