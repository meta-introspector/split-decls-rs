// Generated macro for impl_3185 (impl)
macro_rules! Depcrate_third_party_alga_alga_matriximpl_3185 {
() => {
// Module: crate::third_party::alga::alga_matrix
// Provides: {"impl_3185"}
// Dependencies: {}
impl < T , R : DimName , C : DimName > FiniteDimVectorSpace for OMatrix < T , R , C > where T : Scalar + Field , DefaultAllocator : Allocator < R , C > , { # [inline] fn dimension () -> usize { R :: dim () * C :: dim () } # [inline] fn canonical_basis_element (i : usize) -> Self { assert ! (i < Self :: dimension () , "Index out of bound.") ; let mut res = Self :: zero () ; unsafe { * res . data . get_unchecked_linear_mut (i) = T :: one () ; } res } # [inline] fn dot (& self , other : & Self) -> T { self . dot (other) } # [inline] unsafe fn component_unchecked (& self , i : usize) -> & T { self . data . get_unchecked_linear (i) } # [inline] unsafe fn component_unchecked_mut (& mut self , i : usize) -> & mut T { self . data . get_unchecked_linear_mut (i) } }
};
}
