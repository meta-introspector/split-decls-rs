// Generated macro for impl_428 (impl)
macro_rules! Depcrate_coord_translateimpl_428 {
() => {
// Module: crate::coord::translate
// Provides: {"impl_428"}
// Dependencies: {}
impl < C , T > CoordTranslate for T where C : CoordTranslate , T : Deref < Target = C > , { type From = C :: From ; fn translate (& self , from : & Self :: From) -> BackendCoord { self . deref () . translate (from) } }
};
}
