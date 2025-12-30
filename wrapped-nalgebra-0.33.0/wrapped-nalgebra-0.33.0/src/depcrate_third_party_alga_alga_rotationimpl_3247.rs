// Generated macro for impl_3247 (impl)
macro_rules! Depcrate_third_party_alga_alga_rotationimpl_3247 {
() => {
// Module: crate::third_party::alga::alga_rotation
// Provides: {"impl_3247"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , const D : usize > AffineTransformation < Point < T , D > > for Rotation < T , D > { type Rotation = Self ; type NonUniformScaling = Id ; type Translation = Id ; # [inline] fn decompose (& self) -> (Id , Self , Id , Self) { (Id :: new () , * self , Id :: new () , Self :: identity ()) } # [inline] fn append_translation (& self , _ : & Self :: Translation) -> Self { * self } # [inline] fn prepend_translation (& self , _ : & Self :: Translation) -> Self { * self } # [inline] fn append_rotation (& self , r : & Self :: Rotation) -> Self { r * self } # [inline] fn prepend_rotation (& self , r : & Self :: Rotation) -> Self { self * r } # [inline] fn append_scaling (& self , _ : & Self :: NonUniformScaling) -> Self { * self } # [inline] fn prepend_scaling (& self , _ : & Self :: NonUniformScaling) -> Self { * self } }
};
}
