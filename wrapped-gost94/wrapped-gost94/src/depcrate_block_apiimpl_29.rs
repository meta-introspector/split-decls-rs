// Generated macro for impl_29 (impl)
macro_rules! Depcrate_block_apiimpl_29 {
() => {
// Module: crate::block_api
// Provides: {"impl_29"}
// Dependencies: {}
impl < P : Gost94Params > fmt :: Debug for Gost94Core < P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str (P :: NAME) ? ; f . write_str ("Core { .. }") } }
};
}
