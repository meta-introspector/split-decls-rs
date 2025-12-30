// Generated macro for impl_3163 (impl)
macro_rules! Depcrate_third_party_alga_alga_isometryimpl_3163 {
() => {
// Module: crate::third_party::alga::alga_isometry
// Provides: {"impl_3163"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , R , const D : usize > AffineTransformation < Point < T , D > > for Isometry < T , R , D > where R : Rotation < Point < T , D > > + AbstractRotation < T , D > , { type Rotation = R ; type NonUniformScaling = Id ; type Translation = Translation < T , D > ; # [inline] fn decompose (& self) -> (Self :: Translation , R , Id , R) { (self . translation , self . rotation . clone () , Id :: new () , < R as AbstractRotation < T , D > > :: identity () ,) } # [inline] fn append_translation (& self , t : & Self :: Translation) -> Self { t * self } # [inline] fn prepend_translation (& self , t : & Self :: Translation) -> Self { self * t } # [inline] fn append_rotation (& self , r : & Self :: Rotation) -> Self { let shift = Transformation :: transform_vector (r , & self . translation . vector) ; Isometry :: from_parts (Translation :: from (shift) , r . clone () * self . rotation . clone ()) } # [inline] fn prepend_rotation (& self , r : & Self :: Rotation) -> Self { Isometry :: from_parts (self . translation , self . rotation . prepend_rotation (r)) } # [inline] fn append_scaling (& self , _ : & Self :: NonUniformScaling) -> Self { self . clone () } # [inline] fn prepend_scaling (& self , _ : & Self :: NonUniformScaling) -> Self { self . clone () } # [inline] fn append_rotation_wrt_point (& self , r : & Self :: Rotation , p : & Point < T , D >) -> Option < Self > { let mut res = self . clone () ; res . append_rotation_wrt_point_mut (r , p) ; Some (res) } }
};
}
