// Generated macro for pspecified_algorithm_identifier (function)
macro_rules! Depcrate_paramspspecified_algorithm_identifier {
() => {
// Module: crate::params
// Provides: {"pspecified_algorithm_identifier"}
// Dependencies: {}
fn pspecified_algorithm_identifier (label : & impl AsRef < [u8] >) -> AlgorithmIdentifierRef < '_ > { AlgorithmIdentifierRef { oid : OID_PSPECIFIED , parameters : Some (AnyRef :: new (Tag :: OctetString , label . as_ref ()) . expect ("error creating OAEP params") ,) , } }
};
}
