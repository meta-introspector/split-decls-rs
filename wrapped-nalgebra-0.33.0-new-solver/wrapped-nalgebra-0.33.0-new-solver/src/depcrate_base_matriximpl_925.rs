// Generated macro for impl_925 (impl)
macro_rules! Depcrate_base_matriximpl_925 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_925"}
// Dependencies: {}
impl < T : Scalar + Zero , D : DimAdd < U1 > , S : RawStorage < T , D > > Vector < T , D , S > { # [doc = " Computes the coordinates in projective space of this vector, i.e., appends a `0` to its"] # [doc = " coordinates."] # [inline] # [must_use] pub fn to_homogeneous (& self) -> OVector < T , DimSum < D , U1 > > where DefaultAllocator : Allocator < DimSum < D , U1 > > , { self . push (T :: zero ()) } # [doc = " Constructs a vector from coordinates in projective space, i.e., removes a `0` at the end of"] # [doc = " `self`. Returns `None` if this last component is not zero."] # [inline] pub fn from_homogeneous < SB > (v : Vector < T , DimSum < D , U1 > , SB >) -> Option < OVector < T , D > > where SB : RawStorage < T , DimSum < D , U1 > > , DefaultAllocator : Allocator < D > , { if v [v . len () - 1] . is_zero () { let nrows = D :: from_usize (v . len () - 1) ; Some (v . generic_view ((0 , 0) , (nrows , Const :: < 1 >)) . into_owned ()) } else { None } } }
};
}
