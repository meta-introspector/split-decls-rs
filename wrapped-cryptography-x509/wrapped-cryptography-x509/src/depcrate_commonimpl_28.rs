// Generated macro for impl_28 (impl)
macro_rules! Depcrate_commonimpl_28 {
() => {
// Module: crate::common
// Provides: {"impl_28"}
// Dependencies: {}
impl < T , U > Asn1ReadableOrWritable < T , U > { pub fn new_read (v : T) -> Self { Asn1ReadableOrWritable :: Read (v) } pub fn new_write (v : U) -> Self { Asn1ReadableOrWritable :: Write (v) } pub fn unwrap_read (& self) -> & T { match self { Asn1ReadableOrWritable :: Read (v) => v , Asn1ReadableOrWritable :: Write (_) => panic ! ("unwrap_read called on a Write value") , } } }
};
}
