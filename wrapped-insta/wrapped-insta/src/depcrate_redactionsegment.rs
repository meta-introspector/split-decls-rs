// Generated macro for Segment (enum)
macro_rules! Depcrate_redactionSegment {
() => {
// Module: crate::redaction
// Provides: {"Segment"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq)] pub enum Segment < 'a > { DeepWildcard , Wildcard , Key (Cow < 'a , str >) , Index (u64) , Range (Option < i64 > , Option < i64 >) , }
};
}
