// Generated macro for impl_2999 (impl)
macro_rules! Depcrate_linalg_uduimpl_2999 {
() => {
// Module: crate::linalg::udu
// Provides: {"impl_2999"}
// Dependencies: {}
impl < T : RealField , D : Dim > UDU < T , D > where DefaultAllocator : Allocator < D > + Allocator < D , D > , { # [doc = " Computes the UDU^T factorization."] # [doc = ""] # [doc = " The input matrix `p` is assumed to be symmetric and this decomposition will only read"] # [doc = " the upper-triangular part of `p`."] # [doc = ""] # [doc = " Ref.: \"Optimal control and estimation-Dover Publications\", Robert F. Stengel, (1994) page 360"] pub fn new (p : OMatrix < T , D , D >) -> Option < Self > { let n = p . ncols () ; let n_dim = p . shape_generic () . 1 ; let mut d = OVector :: zeros_generic (n_dim , Const :: < 1 >) ; let mut u = OMatrix :: zeros_generic (n_dim , n_dim) ; d [n - 1] = p [(n - 1 , n - 1)] . clone () ; if d [n - 1] . is_zero () { return None ; } u . column_mut (n - 1) . axpy (T :: one () / d [n - 1] . clone () , & p . column (n - 1) , T :: zero ()) ; for j in (0 .. n - 1) . rev () { let mut d_j = d [j] . clone () ; for k in j + 1 .. n { d_j += d [k] . clone () * u [(j , k)] . clone () . powi (2) ; } d [j] = p [(j , j)] . clone () - d_j ; if d [j] . is_zero () { return None ; } for i in (0 ..= j) . rev () { let mut u_ij = u [(i , j)] . clone () ; for k in j + 1 .. n { u_ij += d [k] . clone () * u [(j , k)] . clone () * u [(i , k)] . clone () ; } u [(i , j)] = (p [(i , j)] . clone () - u_ij) / d [j] . clone () ; } u [(j , j)] = T :: one () ; } Some (Self { u , d }) } # [doc = " Returns the diagonal elements as a matrix"] # [must_use] pub fn d_matrix (& self) -> OMatrix < T , D , D > { OMatrix :: from_diagonal (& self . d) } }
};
}
