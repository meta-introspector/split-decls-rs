// Generated macro for impl_3146 (impl)
macro_rules! Depcrate_third_party_alga_alga_dual_quaternionimpl_3146 {
() => {
// Module: crate::third_party::alga::alga_dual_quaternion
// Provides: {"impl_3146"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > AffineTransformation < Point3 < T > > for UnitDualQuaternion < T > { type Rotation = UnitQuaternion < T > ; type NonUniformScaling = Id ; type Translation = Translation3 < T > ; # [inline] fn decompose (& self) -> (Self :: Translation , Self :: Rotation , Id , Self :: Rotation) { (self . translation () , self . rotation () , Id :: new () , UnitQuaternion :: identity () ,) } # [inline] fn append_translation (& self , translation : & Self :: Translation) -> Self { self * Self :: from_parts (* translation , UnitQuaternion :: identity ()) } # [inline] fn prepend_translation (& self , translation : & Self :: Translation) -> Self { Self :: from_parts (* translation , UnitQuaternion :: identity ()) * self } # [inline] fn append_rotation (& self , r : & Self :: Rotation) -> Self { r * self } # [inline] fn prepend_rotation (& self , r : & Self :: Rotation) -> Self { self * r } # [inline] fn append_scaling (& self , _ : & Self :: NonUniformScaling) -> Self { * self } # [inline] fn prepend_scaling (& self , _ : & Self :: NonUniformScaling) -> Self { * self } }
};
}
