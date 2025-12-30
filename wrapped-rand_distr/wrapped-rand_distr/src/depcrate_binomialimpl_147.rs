// Generated macro for impl_147 (impl)
macro_rules! Depcrate_binomialimpl_147 {
() => {
// Module: crate::binomial
// Provides: {"impl_147"}
// Dependencies: {}
impl Distribution < u64 > for Binomial { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> u64 { match self . method { Method :: Binv (binv_para , flipped) => binv (binv_para , flipped , rng) , Method :: Btpe (btpe_para , flipped) => btpe (btpe_para , flipped , rng) , Method :: Poisson (poisson) => poisson . sample (rng) as u64 , Method :: Constant (c) => c , } } }
};
}
