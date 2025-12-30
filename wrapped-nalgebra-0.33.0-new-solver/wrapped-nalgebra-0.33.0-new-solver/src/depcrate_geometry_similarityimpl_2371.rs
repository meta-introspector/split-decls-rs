// Generated macro for impl_2371 (impl)
macro_rules! Depcrate_geometry_similarityimpl_2371 {
() => {
// Module: crate::geometry::similarity
// Provides: {"impl_2371"}
// Dependencies: {}
impl < T : Scalar + Zero , R , const D : usize > Similarity < T , R , D > where R : AbstractRotation < T , D > , { # [doc = " Creates a new similarity from its rotational and translational parts."] # [inline] pub fn from_parts (translation : Translation < T , D > , rotation : R , scaling : T) -> Self { Self :: from_isometry (Isometry :: from_parts (translation , rotation) , scaling) } # [doc = " Creates a new similarity from its rotational and translational parts."] # [inline] pub fn from_isometry (isometry : Isometry < T , R , D > , scaling : T) -> Self { assert ! (! scaling . is_zero () , "The scaling factor must not be zero.") ; Self { isometry , scaling } } # [doc = " The scaling factor of this similarity transformation."] # [inline] pub fn set_scaling (& mut self , scaling : T) { assert ! (! scaling . is_zero () , "The similarity scaling factor must not be zero.") ; self . scaling = scaling ; } }
};
}
