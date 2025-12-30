// Generated macro for impl_3296 (impl)
macro_rules! Depcrate_third_party_alga_alga_translationimpl_3296 {
() => {
// Module: crate::third_party::alga::alga_translation
// Provides: {"impl_3296"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , const D : usize > AffineTransformation < Point < T , D > > for Translation < T , D > { type Rotation = Id ; type NonUniformScaling = Id ; type Translation = Self ; # [inline] fn decompose (& self) -> (Self , Id , Id , Id) { (* self , Id :: new () , Id :: new () , Id :: new ()) } # [inline] fn append_translation (& self , t : & Self :: Translation) -> Self { t * self } # [inline] fn prepend_translation (& self , t : & Self :: Translation) -> Self { self * t } # [inline] fn append_rotation (& self , _ : & Self :: Rotation) -> Self { * self } # [inline] fn prepend_rotation (& self , _ : & Self :: Rotation) -> Self { * self } # [inline] fn append_scaling (& self , _ : & Self :: NonUniformScaling) -> Self { * self } # [inline] fn prepend_scaling (& self , _ : & Self :: NonUniformScaling) -> Self { * self } }
};
}
