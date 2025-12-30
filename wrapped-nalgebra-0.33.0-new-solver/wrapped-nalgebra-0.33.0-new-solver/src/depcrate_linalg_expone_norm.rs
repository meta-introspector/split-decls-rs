// Generated macro for one_norm (function)
macro_rules! Depcrate_linalg_expone_norm {
() => {
// Module: crate::linalg::exp
// Provides: {"one_norm"}
// Dependencies: {}
fn one_norm < T , D > (m : & OMatrix < T , D , D >) -> T :: RealField where T : ComplexField , D : Dim , DefaultAllocator : Allocator < D , D > , { let mut max = < T as ComplexField > :: RealField :: zero () ; for i in 0 .. m . ncols () { let col = m . column (i) ; max = max . max (col . iter () . fold (< T as ComplexField > :: RealField :: zero () , | a , b | { a + b . clone () . abs () }) ,) ; } max }
};
}
