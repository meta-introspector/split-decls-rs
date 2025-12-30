// Generated macro for impl_3202 (impl)
macro_rules! Depcrate_third_party_alga_alga_pointimpl_3202 {
() => {
// Module: crate::third_party::alga::alga_point
// Provides: {"impl_3202"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , const D : usize > EuclideanSpace for Point < T , D > { type Coordinates = SVector < T , D > ; type RealField = T ; # [inline] fn origin () -> Self { Self :: origin () } # [inline] fn coordinates (& self) -> Self :: Coordinates { self . coords } # [inline] fn from_coordinates (coords : Self :: Coordinates) -> Self { Self :: from (coords) } # [inline] fn scale_by (& self , n : T) -> Self { self * n } }
};
}
