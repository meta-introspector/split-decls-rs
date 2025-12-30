// Generated macro for impl_665 (impl)
macro_rules! Depcrate_base_cgimpl_665 {
() => {
// Module: crate::base::cg
// Provides: {"impl_665"}
// Dependencies: {}
# [doc = " # Translation and scaling in any dimension"] impl < T , D : DimName > OMatrix < T , D , D > where T : Scalar + Zero + One , DefaultAllocator : Allocator < D , D > , { # [doc = " Creates a new homogeneous matrix that applies the same scaling factor on each dimension."] # [inline] pub fn new_scaling (scaling : T) -> Self { let mut res = Self :: from_diagonal_element (scaling) ; res [(D :: dim () - 1 , D :: dim () - 1)] = T :: one () ; res } # [doc = " Creates a new homogeneous matrix that applies a distinct scaling factor for each dimension."] # [inline] pub fn new_nonuniform_scaling < SB > (scaling : & Vector < T , DimNameDiff < D , U1 > , SB >) -> Self where D : DimNameSub < U1 > , SB : Storage < T , DimNameDiff < D , U1 > > , { let mut res = Self :: identity () ; for i in 0 .. scaling . len () { res [(i , i)] = scaling [i] . clone () ; } res } # [doc = " Creates a new homogeneous matrix that applies a pure translation."] # [inline] pub fn new_translation < SB > (translation : & Vector < T , DimNameDiff < D , U1 > , SB >) -> Self where D : DimNameSub < U1 > , SB : Storage < T , DimNameDiff < D , U1 > > , { let mut res = Self :: identity () ; res . generic_view_mut ((0 , D :: dim () - 1) , (DimNameDiff :: < D , U1 > :: name () , Const :: < 1 >) ,) . copy_from (translation) ; res } }
};
}
