// Generated macro for impl_9 (impl)
macro_rules! Depcrate_asn1_typeimpl_9 {
() => {
// Module: crate::asn1_type
// Provides: {"impl_9"}
// Dependencies: {}
impl fmt :: Display for Asn1Type { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Asn1Type :: BitString => "BIT STRING" , Asn1Type :: Ia5String => "IA5String" , Asn1Type :: GeneralizedTime => "GeneralizedTime" , Asn1Type :: OctetString => "OCTET STRING" , Asn1Type :: PrintableString => "PrintableString" , Asn1Type :: TeletexString => "TeletexString" , Asn1Type :: VideotexString => "VideotexString" , Asn1Type :: UtcTime => "UTCTime" , Asn1Type :: Utf8String => "UTF8String" , Asn1Type :: BmpString => "BMPString" , }) } }
};
}
