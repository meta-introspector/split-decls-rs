// Generated macro for impl_26 (impl)
macro_rules! Depcrate_commonimpl_26 {
() => {
// Module: crate::common
// Provides: {"impl_26"}
// Dependencies: {}
impl Time { pub fn as_datetime (& self) -> & asn1 :: DateTime { match self { Time :: UtcTime (data) => data . as_datetime () , Time :: GeneralizedTime (data) => data . as_datetime () , } } }
};
}
