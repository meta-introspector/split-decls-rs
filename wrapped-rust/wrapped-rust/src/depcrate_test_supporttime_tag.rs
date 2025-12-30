// Generated macro for time_tag (function)
macro_rules! Depcrate_test_supporttime_tag {
() => {
// Module: crate::test_support
// Provides: {"time_tag"}
// Dependencies: {}
fn time_tag (t : & Time) -> u8 { match t { Time :: UtcTime (_) => asn1 :: UtcTime :: TAG . as_u8 () . unwrap () , Time :: GeneralizedTime (_) => asn1 :: GeneralizedTime :: TAG . as_u8 () . unwrap () , } }
};
}
