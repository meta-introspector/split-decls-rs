// Generated macro for impl_173 (impl)
macro_rules! Depcrate_nameimpl_173 {
() => {
// Module: crate::name
// Provides: {"impl_173"}
// Dependencies: {}
impl asn1 :: SimpleAsn1Writable for UnvalidatedIA5String < '_ > { const TAG : asn1 :: Tag = asn1 :: IA5String :: TAG ; fn write_data (& self , dest : & mut asn1 :: WriteBuf) -> asn1 :: WriteResult { dest . push_slice (self . 0 . as_bytes ()) } fn data_length (& self) -> Option < usize > { Some (self . 0 . len ()) } }
};
}
