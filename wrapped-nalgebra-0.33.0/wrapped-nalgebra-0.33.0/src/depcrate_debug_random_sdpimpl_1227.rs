// Generated macro for impl_1227 (impl)
macro_rules! Depcrate_debug_random_sdpimpl_1227 {
() => {
// Module: crate::debug::random_sdp
// Provides: {"impl_1227"}
// Dependencies: {}
impl < T : ComplexField , D : Dim > RandomSDP < T , D > where DefaultAllocator : Allocator < D , D > , { # [doc = " Retrieve the generated matrix."] pub fn unwrap (self) -> OMatrix < T , D , D > { self . m } # [doc = " Creates a new well conditioned symmetric definite-positive matrix from its dimension and a"] # [doc = " random reals generators."] pub fn new < Rand : FnMut () -> T > (dim : D , mut rand : Rand) -> Self { let mut m = RandomOrthogonal :: new (dim , | | rand ()) . unwrap () ; let mt = m . adjoint () ; for i in 0 .. dim . value () { let mut col = m . column_mut (i) ; let eigenval = T :: one () + T :: from_real (rand () . modulus ()) ; col *= eigenval ; } RandomSDP { m : m * mt } } }
};
}
