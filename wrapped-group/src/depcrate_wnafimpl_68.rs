// Generated macro for impl_68 (impl)
macro_rules! Depcrate_wnafimpl_68 {
() => {
// Module: crate::wnaf
// Provides: {"impl_68"}
// Dependencies: {}
impl < F : PrimeField , const WINDOW_SIZE : usize > WnafScalar < F , WINDOW_SIZE > { # [doc = " Computes the w-NAF representation of the given scalar with the specified"] # [doc = " `WINDOW_SIZE`."] pub fn new (scalar : & F) -> Self { let mut wnaf = vec ! [] ; wnaf_form (& mut wnaf , scalar . to_repr () , WINDOW_SIZE) ; WnafScalar { wnaf , field : PhantomData :: default () , } } }
};
}
