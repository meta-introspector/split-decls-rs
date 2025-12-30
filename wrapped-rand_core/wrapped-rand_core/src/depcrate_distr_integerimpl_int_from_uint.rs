// Generated macro for impl_int_from_uint (macro)
macro_rules! Depcrate_distr_integerimpl_int_from_uint {
() => {
// Module: crate::distr::integer
// Provides: {"impl_int_from_uint"}
// Dependencies: {}
macro_rules ! impl_int_from_uint { ($ ty : ty , $ uty : ty) => { impl Distribution <$ ty > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> $ ty { rng . random ::<$ uty > () as $ ty } } } ; }
};
}
