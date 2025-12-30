// Generated macro for impl_29 (impl)
macro_rules! Depcrate_commonimpl_29 {
() => {
// Module: crate::common
// Provides: {"impl_29"}
// Dependencies: {}
impl < 'a , T : asn1 :: SimpleAsn1Readable < 'a > , U > asn1 :: SimpleAsn1Readable < 'a > for Asn1ReadableOrWritable < T , U > { const TAG : asn1 :: Tag = T :: TAG ; fn parse_data (data : & 'a [u8]) -> asn1 :: ParseResult < Self > { Ok (Self :: new_read (T :: parse_data (data) ?)) } }
};
}
