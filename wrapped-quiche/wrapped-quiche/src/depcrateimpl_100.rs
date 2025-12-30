// Generated macro for impl_100 (impl)
macro_rules! Depcrateimpl_100 {
() => {
// Module: crate
// Provides: {"impl_100"}
// Dependencies: {}
impl < 'a > Iterator for UnknownTransportParameterIterator < 'a > { type Item = & 'a UnknownTransportParameter < Vec < u8 > > ; fn next (& mut self) -> Option < Self :: Item > { let result = self . parameters . get (self . index) ; self . index += 1 ; result } }
};
}
