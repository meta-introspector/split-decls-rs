// Generated macro for impl_669 (impl)
macro_rules! Depcrate_base_cgimpl_669 {
() => {
// Module: crate::base::cg
// Provides: {"impl_669"}
// Dependencies: {}
# [doc = " # Transformation of vectors and points"] impl < T : RealField , D : DimNameSub < U1 > , S : Storage < T , D , D > > SquareMatrix < T , D , S > where DefaultAllocator : Allocator < D , D > + Allocator < DimNameDiff < D , U1 > > + Allocator < DimNameDiff < D , U1 > , DimNameDiff < D , U1 > > , { # [doc = " Transforms the given vector, assuming the matrix `self` uses homogeneous coordinates."] # [inline] pub fn transform_vector (& self , v : & OVector < T , DimNameDiff < D , U1 > > ,) -> OVector < T , DimNameDiff < D , U1 > > { let transform = self . generic_view ((0 , 0) , (DimNameDiff :: < D , U1 > :: name () , DimNameDiff :: < D , U1 > :: name ()) ,) ; let normalizer = self . generic_view ((D :: dim () - 1 , 0) , (Const :: < 1 > , DimNameDiff :: < D , U1 > :: name ()) ,) ; let n = normalizer . tr_dot (v) ; if ! n . is_zero () { return transform * (v / n) ; } transform * v } }
};
}
