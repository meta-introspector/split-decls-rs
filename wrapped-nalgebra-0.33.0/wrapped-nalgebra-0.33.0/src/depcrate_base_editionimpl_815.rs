// Generated macro for impl_815 (impl)
macro_rules! Depcrate_base_editionimpl_815 {
() => {
// Module: crate::base::edition
// Provides: {"impl_815"}
// Dependencies: {}
# [doc = " # Triangular matrix extraction"] impl < T : Scalar + Zero , R : Dim , C : Dim , S : Storage < T , R , C > > Matrix < T , R , C , S > { # [doc = " Extracts the upper triangular part of this matrix (including the diagonal)."] # [inline] # [must_use] pub fn upper_triangle (& self) -> OMatrix < T , R , C > where DefaultAllocator : Allocator < R , C > , { let mut res = self . clone_owned () ; res . fill_lower_triangle (T :: zero () , 1) ; res } # [doc = " Extracts the lower triangular part of this matrix (including the diagonal)."] # [inline] # [must_use] pub fn lower_triangle (& self) -> OMatrix < T , R , C > where DefaultAllocator : Allocator < R , C > , { let mut res = self . clone_owned () ; res . fill_upper_triangle (T :: zero () , 1) ; res } }
};
}
