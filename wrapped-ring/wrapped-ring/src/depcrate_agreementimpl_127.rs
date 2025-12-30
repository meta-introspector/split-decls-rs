// Generated macro for impl_127 (impl)
macro_rules! Depcrate_agreementimpl_127 {
() => {
// Module: crate::agreement
// Provides: {"impl_127"}
// Dependencies: {}
impl core :: fmt :: Debug for PublicKey { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> Result < () , core :: fmt :: Error > { f . debug_struct ("PublicKey") . field ("algorithm" , & self . algorithm) . field ("bytes" , & debug :: HexStr (self . as_ref ())) . finish () } }
};
}
