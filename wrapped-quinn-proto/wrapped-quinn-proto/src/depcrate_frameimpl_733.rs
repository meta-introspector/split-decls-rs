// Generated macro for impl_733 (impl)
macro_rules! Depcrate_frameimpl_733 {
() => {
// Module: crate::frame
// Provides: {"impl_733"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a Ack { type Item = RangeInclusive < u64 > ; type IntoIter = AckIter < 'a > ; fn into_iter (self) -> AckIter < 'a > { AckIter :: new (self . largest , & self . additional [..]) } }
};
}
