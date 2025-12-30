// Generated macro for impl_322 (impl)
macro_rules! Depcrate_testsimpl_322 {
() => {
// Module: crate::tests
// Provides: {"impl_322"}
// Dependencies: {}
impl distributions :: Distribution < DecodePaddingMode > for distributions :: Standard { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> DecodePaddingMode { match rng . gen_range (0 ..= 2) { 0 => DecodePaddingMode :: Indifferent , 1 => DecodePaddingMode :: RequireCanonical , _ => DecodePaddingMode :: RequireNone , } } }
};
}
