// Generated macro for impl_47 (impl)
macro_rules! Depcrate_blockimpl_47 {
() => {
// Module: crate::block
// Provides: {"impl_47"}
// Dependencies: {}
impl BlockExt for Option < Block < '_ > > { fn inner_if_some (& self , area : Rect) -> Rect { self . as_ref () . map_or (area , | block | block . inner (area)) } }
};
}
