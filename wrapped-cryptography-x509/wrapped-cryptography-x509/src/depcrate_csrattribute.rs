// Generated macro for Attribute (struct)
macro_rules! Depcrate_csrAttribute {
() => {
// Module: crate::csr
// Provides: {"Attribute"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct Attribute < 'a > { pub type_id : asn1 :: ObjectIdentifier , pub values : common :: Asn1ReadableOrWritable < asn1 :: SetOf < 'a , asn1 :: Tlv < 'a > > , asn1 :: SetOfWriter < 'a , common :: RawTlv < 'a > , [common :: RawTlv < 'a > ; 1] > , > , }
};
}
