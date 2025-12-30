// Generated macro for impl_2844 (impl)
macro_rules! Depcrate_linalg_inverseimpl_2844 {
() => {
// Module: crate::linalg::inverse
// Provides: {"impl_2844"}
// Dependencies: {}
impl < T : ComplexField , D : Dim , S : Storage < T , D , D > > SquareMatrix < T , D , S > { # [doc = " Attempts to invert this square matrix."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `self` isn’t a square matrix."] # [inline] # [must_use = "Did you mean to use try_inverse_mut()?"] pub fn try_inverse (self) -> Option < OMatrix < T , D , D > > where DefaultAllocator : Allocator < D , D > , { let mut me = self . into_owned () ; if me . try_inverse_mut () { Some (me) } else { None } } }
};
}
