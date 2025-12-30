// Generated macro for impl_3313 (impl)
macro_rules! Depcrate_third_party_alga_alga_unit_compleximpl_3313 {
() => {
// Module: crate::third_party::alga::alga_unit_complex
// Provides: {"impl_3313"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > AffineTransformation < Point2 < T > > for UnitComplex < T > { type Rotation = Self ; type NonUniformScaling = Id ; type Translation = Id ; # [inline] fn decompose (& self) -> (Id , Self , Id , Self) { (Id :: new () , * self , Id :: new () , Self :: identity ()) } # [inline] fn append_translation (& self , _ : & Self :: Translation) -> Self { * self } # [inline] fn prepend_translation (& self , _ : & Self :: Translation) -> Self { * self } # [inline] fn append_rotation (& self , r : & Self :: Rotation) -> Self { r * self } # [inline] fn prepend_rotation (& self , r : & Self :: Rotation) -> Self { self * r } # [inline] fn append_scaling (& self , _ : & Self :: NonUniformScaling) -> Self { * self } # [inline] fn prepend_scaling (& self , _ : & Self :: NonUniformScaling) -> Self { * self } }
};
}
