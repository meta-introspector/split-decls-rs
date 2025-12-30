// Generated macro for impl_15 (impl)
macro_rules! Depcrate_errorimpl_15 {
() => {
// Module: crate::error
// Provides: {"impl_15"}
// Dependencies: {}
impl From < Error > for spki :: Error { fn from (err : Error) -> spki :: Error { match err { Error :: Asn1 (e) => spki :: Error :: Asn1 (e) , Error :: PublicKey (e) => e , _ => spki :: Error :: KeyMalformed , } } }
};
}
