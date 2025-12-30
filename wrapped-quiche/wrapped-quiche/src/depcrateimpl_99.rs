// Generated macro for impl_99 (impl)
macro_rules! Depcrateimpl_99 {
() => {
// Module: crate
// Provides: {"impl_99"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a UnknownTransportParameters { type IntoIter = UnknownTransportParameterIterator < 'a > ; type Item = & 'a UnknownTransportParameter < Vec < u8 > > ; fn into_iter (self) -> Self :: IntoIter { UnknownTransportParameterIterator { index : 0 , parameters : & self . parameters , } } }
};
}
