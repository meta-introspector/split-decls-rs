// Generated macro for impl_3094 (impl)
macro_rules! Depcrate_sparse_cs_matrix_conversionimpl_3094 {
() => {
// Module: crate::sparse::cs_matrix_conversion
// Provides: {"impl_3094"}
// Dependencies: {}
impl < 'a , T : Scalar + Zero , R : Dim , C : Dim , S > From < Matrix < T , R , C , S > > for CsMatrix < T , R , C > where S : Storage < T , R , C > , DefaultAllocator : Allocator < R , C > + Allocator < C > , { fn from (m : Matrix < T , R , C , S >) -> Self { let (nrows , ncols) = m . data . shape () ; let len = m . iter () . filter (| e | ! e . is_zero ()) . count () ; let mut res = CsMatrix :: new_uninitialized_generic (nrows , ncols , len) ; let mut nz = 0 ; for j in 0 .. ncols . value () { let column = m . column (j) ; res . data . p [j] = nz ; for i in 0 .. nrows . value () { if ! column [i] . is_zero () { res . data . i [nz] = i ; res . data . vals [nz] = column [i] . clone () ; nz += 1 ; } } } res } }
};
}
