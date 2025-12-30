// Generated macro for impl_3265 (impl)
macro_rules! Depcrate_third_party_alga_alga_similarityimpl_3265 {
() => {
// Module: crate::third_party::alga::alga_similarity
// Provides: {"impl_3265"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , R , const D : usize > AffineTransformation < Point < T , D > > for Similarity < T , R , D > where R : Rotation < Point < T , D > > + AbstractRotation < T , D > , { type NonUniformScaling = T ; type Rotation = R ; type Translation = Translation < T , D > ; # [inline] fn decompose (& self) -> (Translation < T , D > , R , T , R) { (self . isometry . translation , self . isometry . rotation . clone () , self . scaling () , < R as AbstractRotation < T , D > > :: identity () ,) } # [inline] fn append_translation (& self , t : & Self :: Translation) -> Self { t * self } # [inline] fn prepend_translation (& self , t : & Self :: Translation) -> Self { self * t } # [inline] fn append_rotation (& self , r : & Self :: Rotation) -> Self { Similarity :: from_isometry (self . isometry . append_rotation (r) , self . scaling ()) } # [inline] fn prepend_rotation (& self , r : & Self :: Rotation) -> Self { Similarity :: from_isometry (self . isometry . prepend_rotation (r) , self . scaling ()) } # [inline] fn append_scaling (& self , s : & Self :: NonUniformScaling) -> Self { self . append_scaling (* s) } # [inline] fn prepend_scaling (& self , s : & Self :: NonUniformScaling) -> Self { self . prepend_scaling (* s) } # [inline] fn append_rotation_wrt_point (& self , r : & Self :: Rotation , p : & Point < T , D >) -> Option < Self > { let mut res = self . clone () ; res . append_rotation_wrt_point_mut (r , p) ; Some (res) } }
};
}
