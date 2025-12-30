// Generated macro for EnvelopedData (struct)
macro_rules! Depcrate_pkcs7EnvelopedData {
() => {
// Module: crate::pkcs7
// Provides: {"EnvelopedData"}
// Dependencies: {}
# [derive (asn1 :: Asn1Write , asn1 :: Asn1Read)] pub struct EnvelopedData < 'a > { pub version : u8 , pub recipient_infos : common :: Asn1ReadableOrWritable < asn1 :: SetOf < 'a , RecipientInfo < 'a > > , asn1 :: SetOfWriter < 'a , RecipientInfo < 'a > > , > , pub encrypted_content_info : EncryptedContentInfo < 'a > , }
};
}
