// Generated macro for RandomOrthogonal (struct)
macro_rules! Depcrate_debug_random_orthogonalRandomOrthogonal {
() => {
// Module: crate::debug::random_orthogonal
// Provides: {"RandomOrthogonal"}
// Dependencies: {}
# [doc = " A random orthogonal matrix."] # [derive (Clone , Debug)] pub struct RandomOrthogonal < T : Scalar , D : Dim = Dyn > where DefaultAllocator : Allocator < D , D > , { m : OMatrix < T , D , D > , }
};
}
