// Generated macro for impl_8 (impl)
macro_rules! Depcrate_asn1_typeimpl_8 {
() => {
// Module: crate::asn1_type
// Provides: {"impl_8"}
// Dependencies: {}
impl FromStr for Asn1Type { type Err = ParseError ; fn from_str (s : & str) -> Result < Self , ParseError > { match s { "BIT STRING" => Ok (Self :: BitString) , "IA5String" => Ok (Self :: Ia5String) , "GeneralizedTime" => Ok (Self :: GeneralizedTime) , "OCTET STRING" => Ok (Self :: OctetString) , "PrintableString" => Ok (Self :: PrintableString) , "TeletexString" => Ok (Self :: TeletexString) , "VideotexString" => Ok (Self :: VideotexString) , "UTCTime" => Ok (Self :: UtcTime) , "UTF8String" => Ok (Self :: Utf8String) , "BMPString" => Ok (Self :: BmpString) , _ => Err (ParseError) , } } }
};
}
