// Generated macro for impl_188 (impl)
macro_rules! Depcrate_base_dimensionimpl_188 {
() => {
// Module: crate::base::dimension
// Provides: {"impl_188"}
// Dependencies: {}
unsafe impl < const T : usize > Dim for Const < T > { # [inline] fn try_to_usize () -> Option < usize > { Some (T) } # [inline] fn value (& self) -> usize { T } # [inline] fn from_usize (dim : usize) -> Self { assert_eq ! (dim , T) ; Self } }
};
}
