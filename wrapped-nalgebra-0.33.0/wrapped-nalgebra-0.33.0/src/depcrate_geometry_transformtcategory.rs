// Generated macro for TCategory (trait)
macro_rules! Depcrate_geometry_transformTCategory {
() => {
// Module: crate::geometry::transform
// Provides: {"TCategory"}
// Dependencies: {}
# [doc = " Trait implemented by phantom types identifying the projective transformation type."] # [doc = ""] # [doc = " NOTE: this trait is not intended to be implemented outside of the `nalgebra` crate."] pub trait TCategory : Any + Debug + Copy + PartialEq + Send { # [doc = " Indicates whether a `Transform` with the category `Self` has a bottom-row different from"] # [doc = " `0 0 .. 1`."] # [inline] fn has_normalizer () -> bool { true } # [doc = " Checks that the given matrix is a valid homogeneous representation of an element of the"] # [doc = " category `Self`."] fn check_homogeneous_invariants < T : RealField , D : DimName > (mat : & OMatrix < T , D , D >) -> bool where T :: Epsilon : Clone , DefaultAllocator : Allocator < D , D > ; }
};
}
