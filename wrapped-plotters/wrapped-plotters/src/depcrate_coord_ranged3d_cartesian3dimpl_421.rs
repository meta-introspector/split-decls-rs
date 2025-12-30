// Generated macro for impl_421 (impl)
macro_rules! Depcrate_coord_ranged3d_cartesian3dimpl_421 {
() => {
// Module: crate::coord::ranged3d::cartesian3d
// Provides: {"impl_421"}
// Dependencies: {}
impl < X : Ranged , Y : Ranged , Z : Ranged > CoordTranslate for Cartesian3d < X , Y , Z > { type From = (X :: ValueType , Y :: ValueType , Z :: ValueType) ; fn translate (& self , coord : & Self :: From) -> BackendCoord { let pixel_coord_3d = self . map_3d (& coord . 0 , & coord . 1 , & coord . 2) ; self . projection * pixel_coord_3d } fn depth (& self , coord : & Self :: From) -> i32 { self . projected_depth (& coord . 0 , & coord . 1 , & coord . 2) } }
};
}
