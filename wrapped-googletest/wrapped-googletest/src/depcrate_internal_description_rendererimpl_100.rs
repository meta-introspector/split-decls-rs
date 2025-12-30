// Generated macro for impl_100 (impl)
macro_rules! Depcrate_internal_description_rendererimpl_100 {
() => {
// Module: crate::internal::description_renderer
// Provides: {"impl_100"}
// Dependencies: {}
impl From < String > for Block { fn from (value : String) -> Self { Block :: Literal (value . lines () . map (| v | Fragment (v . to_string () . into ())) . collect ()) } }
};
}
