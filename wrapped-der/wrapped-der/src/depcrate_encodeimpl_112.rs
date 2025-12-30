// Generated macro for impl_112 (impl)
macro_rules! Depcrate_encodeimpl_112 {
() => {
// Module: crate::encode
// Provides: {"impl_112"}
// Dependencies: {}
impl < T > Encode for T where T : EncodeValue + Tagged + ? Sized , { # [doc = " Compute the length of this TLV object in bytes when encoded as ASN.1 DER."] fn encoded_len (& self) -> Result < Length > { self . value_len () . and_then (| len | len . for_tlv (self . tag ())) } # [doc = " Encode this TLV object as ASN.1 DER using the provided [`Writer`]."] fn encode (& self , writer : & mut impl Writer) -> Result < () > { self . header () ? . encode (writer) ? ; self . encode_value (writer) } }
};
}
