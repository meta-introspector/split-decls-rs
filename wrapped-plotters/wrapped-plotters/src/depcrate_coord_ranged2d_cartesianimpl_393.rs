// Generated macro for impl_393 (impl)
macro_rules! Depcrate_coord_ranged2d_cartesianimpl_393 {
() => {
// Module: crate::coord::ranged2d::cartesian
// Provides: {"impl_393"}
// Dependencies: {}
impl < X : ReversibleRanged , Y : ReversibleRanged > ReverseCoordTranslate for Cartesian2d < X , Y > { fn reverse_translate (& self , input : BackendCoord) -> Option < Self :: From > { Some ((self . logic_x . unmap (input . 0 , self . back_x) ? , self . logic_y . unmap (input . 1 , self . back_y) ? ,)) } }
};
}
