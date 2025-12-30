// Generated macro for impl_307 (impl)
macro_rules! Depcrate_normalimpl_307 {
() => {
// Module: crate::normal
// Provides: {"impl_307"}
// Dependencies: {}
impl < F > Distribution < F > for Normal < F > where F : Float , StandardNormal : Distribution < F > , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { self . from_zscore (rng . sample (StandardNormal)) } }
};
}
