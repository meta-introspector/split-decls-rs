// Generated macro for AttributeSet (enum)
macro_rules! Depcrate_pkcs12AttributeSet {
() => {
// Module: crate::pkcs12
// Provides: {"AttributeSet"}
// Dependencies: {}
# [derive (asn1 :: Asn1DefinedByWrite)] pub enum AttributeSet < 'a > { # [defined_by (FRIENDLY_NAME_OID)] FriendlyName (asn1 :: SetOfWriter < 'a , Utf8StoredBMPString < 'a > , [Utf8StoredBMPString < 'a > ; 1] >) , # [defined_by (LOCAL_KEY_ID_OID)] LocalKeyId (asn1 :: SetOfWriter < 'a , & 'a [u8] , [& 'a [u8] ; 1] >) , # [defined_by (JDK_TRUSTSTORE_USAGE)] JDKTruststoreUsage (asn1 :: SetOfWriter < 'a , ObjectIdentifier , [ObjectIdentifier ; 1] >) , }
};
}
