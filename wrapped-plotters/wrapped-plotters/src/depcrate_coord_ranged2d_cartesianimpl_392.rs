// Generated macro for impl_392 (impl)
macro_rules! Depcrate_coord_ranged2d_cartesianimpl_392 {
() => {
// Module: crate::coord::ranged2d::cartesian
// Provides: {"impl_392"}
// Dependencies: {}
impl < X : Ranged , Y : Ranged > CoordTranslate for Cartesian2d < X , Y > { type From = (X :: ValueType , Y :: ValueType) ; fn translate (& self , from : & Self :: From) -> BackendCoord { (self . logic_x . map (& from . 0 , self . back_x) , self . logic_y . map (& from . 1 , self . back_y) ,) } }
};
}
