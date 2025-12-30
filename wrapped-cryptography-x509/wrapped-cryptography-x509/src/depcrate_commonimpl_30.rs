// Generated macro for impl_30 (impl)
macro_rules! Depcrate_commonimpl_30 {
() => {
// Module: crate::common
// Provides: {"impl_30"}
// Dependencies: {}
impl < T : asn1 :: SimpleAsn1Writable , U : asn1 :: SimpleAsn1Writable > asn1 :: SimpleAsn1Writable for Asn1ReadableOrWritable < T , U > { const TAG : asn1 :: Tag = U :: TAG ; fn write_data (& self , w : & mut asn1 :: WriteBuf) -> asn1 :: WriteResult { match self { Asn1ReadableOrWritable :: Read (v) => T :: write_data (v , w) , Asn1ReadableOrWritable :: Write (v) => U :: write_data (v , w) , } } fn data_length (& self) -> Option < usize > { match self { Asn1ReadableOrWritable :: Read (v) => T :: data_length (v) , Asn1ReadableOrWritable :: Write (v) => U :: data_length (v) , } } }
};
}
