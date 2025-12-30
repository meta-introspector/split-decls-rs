// Generated macro for impl_23 (impl)
macro_rules! Depcrate_distr_bernoulliimpl_23 {
() => {
// Module: crate::distr::bernoulli
// Provides: {"impl_23"}
// Dependencies: {}
impl Distribution < bool > for Bernoulli { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> bool { if self . p_int == ALWAYS_TRUE { return true ; } let v : u64 = rng . random () ; v < self . p_int } }
};
}
