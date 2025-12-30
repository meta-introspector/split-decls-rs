// Generated macro for impl_1215 (impl)
macro_rules! Depcrate_debug_random_orthogonalimpl_1215 {
() => {
// Module: crate::debug::random_orthogonal
// Provides: {"impl_1215"}
// Dependencies: {}
impl < T : ComplexField , D : Dim > RandomOrthogonal < T , D > where DefaultAllocator : Allocator < D , D > , { # [doc = " Retrieve the generated matrix."] pub fn unwrap (self) -> OMatrix < T , D , D > { self . m } # [doc = " Creates a new random orthogonal matrix from its dimension and a random reals generators."] pub fn new < Rand : FnMut () -> T > (dim : D , mut rand : Rand) -> Self { let mut res = OMatrix :: identity_generic (dim , dim) ; for i in 0 .. dim . value () - 1 { let rot = GivensRotation :: new (rand () , rand ()) . 0 ; rot . rotate (& mut res . fixed_rows_mut :: < 2 > (i)) ; } RandomOrthogonal { m : res } } }
};
}
