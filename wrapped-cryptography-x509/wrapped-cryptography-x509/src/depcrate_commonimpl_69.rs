// Generated macro for impl_69 (impl)
macro_rules! Depcrate_commonimpl_69 {
() => {
// Module: crate::common
// Provides: {"impl_69"}
// Dependencies: {}
impl asn1 :: SimpleAsn1Writable for Utf8StoredBMPString < '_ > { const TAG : asn1 :: Tag = asn1 :: BMPString :: TAG ; fn write_data (& self , writer : & mut asn1 :: WriteBuf) -> asn1 :: WriteResult { for ch in self . 0 . encode_utf16 () { writer . push_slice (& ch . to_be_bytes ()) ? ; } Ok (()) } fn data_length (& self) -> Option < usize > { Some (self . 0 . encode_utf16 () . count () * 2) } }
};
}
