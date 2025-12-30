// Generated macro for impl_131 (impl)
macro_rules! Depcrate_agreementimpl_131 {
() => {
// Module: crate::agreement
// Provides: {"impl_131"}
// Dependencies: {}
impl < B : core :: fmt :: Debug > core :: fmt :: Debug for UnparsedPublicKey < B > where B : AsRef < [u8] > , { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> Result < () , core :: fmt :: Error > { f . debug_struct ("UnparsedPublicKey") . field ("algorithm" , & self . algorithm) . field ("bytes" , & debug :: HexStr (self . bytes . as_ref ())) . finish () } }
};
}
