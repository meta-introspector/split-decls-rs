// Generated macro for impl_nzint (macro)
macro_rules! Depcrate_distr_integerimpl_nzint {
() => {
// Module: crate::distr::integer
// Provides: {"impl_nzint"}
// Dependencies: {}
macro_rules ! impl_nzint { ($ ty : ty , $ new : path) => { impl Distribution <$ ty > for StandardUniform { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> $ ty { loop { if let Some (nz) = $ new (rng . random ()) { break nz ; } } } } } ; }
};
}
