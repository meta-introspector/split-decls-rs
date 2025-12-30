// Generated macro for impl_57 (impl)
macro_rules! Depcrate_crandimpl_57 {
() => {
// Module: crate::crand
// Provides: {"impl_57"}
// Dependencies: {}
impl < T > Distribution < Complex < T > > for Standard where T : Num + Clone , Standard : Distribution < T > , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> Complex < T > { Complex :: new (self . sample (rng) , self . sample (rng)) } }
};
}
