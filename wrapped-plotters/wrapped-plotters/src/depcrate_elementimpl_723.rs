// Generated macro for impl_723 (impl)
macro_rules! Depcrate_elementimpl_723 {
() => {
// Module: crate::element
// Provides: {"impl_723"}
// Dependencies: {}
impl CoordMapper for BackendCoordOnly { type Output = BackendCoord ; fn map < CT : CoordTranslate > (coord_trans : & CT , from : & CT :: From , rect : & Rect) -> BackendCoord { rect . truncate (coord_trans . translate (from)) } }
};
}
