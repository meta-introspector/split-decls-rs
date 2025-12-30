// Generated macro for impl_2922 (impl)
macro_rules! Depcrate_linalg_schurimpl_2922 {
() => {
// Module: crate::linalg::schur
// Provides: {"impl_2922"}
// Dependencies: {}
impl < T : ComplexField , D : Dim , S : Storage < T , D , D > > SquareMatrix < T , D , S > where D : DimSub < U1 > , DefaultAllocator : Allocator < D , DimDiff < D , U1 > > + Allocator < DimDiff < D , U1 > > + Allocator < D , D > + Allocator < D > , { # [doc = " Computes the eigenvalues of this matrix."] # [must_use] pub fn eigenvalues (& self) -> Option < OVector < T , D > > { assert ! (self . is_square () , "Unable to compute eigenvalues of a non-square matrix.") ; let mut work = Matrix :: zeros_generic (self . shape_generic () . 0 , Const :: < 1 >) ; if self . nrows () == 2 { let me = self . fixed_view :: < 2 , 2 > (0 , 0) ; return match compute_2x2_eigvals (& me) { Some ((a , b)) => { work [0] = a ; work [1] = b ; Some (work) } None => None , } ; } let schur = Schur :: do_decompose (self . clone_owned () , & mut work , T :: RealField :: default_epsilon () , 0 , false ,) . unwrap () ; if Schur :: do_eigenvalues (& schur . 1 , & mut work) { Some (work) } else { None } } # [doc = " Computes the eigenvalues of this matrix."] # [must_use] pub fn complex_eigenvalues (& self) -> OVector < NumComplex < T > , D > where T : RealField , DefaultAllocator : Allocator < D > , { let dim = self . shape_generic () . 0 ; let mut work = Matrix :: zeros_generic (dim , Const :: < 1 >) ; let schur = Schur :: do_decompose (self . clone_owned () , & mut work , T :: default_epsilon () , 0 , false ,) . unwrap () ; let mut eig = Matrix :: uninit (dim , Const :: < 1 >) ; Schur :: do_complex_eigenvalues (& schur . 1 , & mut eig) ; unsafe { eig . assume_init () } } }
};
}
