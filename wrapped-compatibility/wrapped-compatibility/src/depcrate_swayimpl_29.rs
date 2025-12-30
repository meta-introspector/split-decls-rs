// Generated macro for impl_29 (impl)
macro_rules! Depcrate_swayimpl_29 {
() => {
// Module: crate::sway
// Provides: {"impl_29"}
// Dependencies: {}
impl Trade { fn random (rng : & mut impl Rng) -> Self { Self { id : rng . gen () , liquidation : rng . gen () , price : rng . gen () , side : if rng . gen () { TradeSide :: Buy } else { TradeSide :: Sell } , size : rng . gen () , time : crate :: gen_string (rng) , } } }
};
}
