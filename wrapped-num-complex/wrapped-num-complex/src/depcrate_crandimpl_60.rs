// Generated macro for impl_60 (impl)
macro_rules! Depcrate_crandimpl_60 {
() => {
// Module: crate::crand
// Provides: {"impl_60"}
// Dependencies: {}
impl < T , Re , Im > Distribution < Complex < T > > for ComplexDistribution < Re , Im > where T : Num + Clone , Re : Distribution < T > , Im : Distribution < T > , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> Complex < T > { Complex :: new (self . re . sample (rng) , self . im . sample (rng)) } }
};
}
