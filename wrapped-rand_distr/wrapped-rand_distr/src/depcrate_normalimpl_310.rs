// Generated macro for impl_310 (impl)
macro_rules! Depcrate_normalimpl_310 {
() => {
// Module: crate::normal
// Provides: {"impl_310"}
// Dependencies: {}
impl < F > Distribution < F > for LogNormal < F > where F : Float , StandardNormal : Distribution < F > , { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { self . norm . sample (rng) . exp () } }
};
}
