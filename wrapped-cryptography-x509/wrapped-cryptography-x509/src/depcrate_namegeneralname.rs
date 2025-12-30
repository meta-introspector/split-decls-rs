// Generated macro for GeneralName (enum)
macro_rules! Depcrate_nameGeneralName {
() => {
// Module: crate::name
// Provides: {"GeneralName"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub enum GeneralName < 'a > { # [implicit (0)] OtherName (OtherName < 'a >) , # [implicit (1)] RFC822Name (UnvalidatedIA5String < 'a >) , # [implicit (2)] DNSName (UnvalidatedIA5String < 'a >) , # [implicit (3)] X400Address (asn1 :: Sequence < 'a >) , # [explicit (4)] DirectoryName (Name < 'a >) , # [implicit (5)] EDIPartyName (asn1 :: Sequence < 'a >) , # [implicit (6)] UniformResourceIdentifier (UnvalidatedIA5String < 'a >) , # [implicit (7)] IPAddress (& 'a [u8]) , # [implicit (8)] RegisteredID (asn1 :: ObjectIdentifier) , }
};
}
