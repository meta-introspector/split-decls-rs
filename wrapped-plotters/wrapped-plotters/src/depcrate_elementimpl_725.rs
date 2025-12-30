// Generated macro for impl_725 (impl)
macro_rules! Depcrate_elementimpl_725 {
() => {
// Module: crate::element
// Provides: {"impl_725"}
// Dependencies: {}
impl CoordMapper for BackendCoordAndZ { type Output = (BackendCoord , i32) ; fn map < CT : CoordTranslate > (coord_trans : & CT , from : & CT :: From , rect : & Rect ,) -> (BackendCoord , i32) { let coord = rect . truncate (coord_trans . translate (from)) ; let z = coord_trans . depth (from) ; (coord , z) } }
};
}
