// Generated macro for pkcs12_attributes (function)
macro_rules! Depcrate_pkcs12pkcs12_attributes {
() => {
// Module: crate::pkcs12
// Provides: {"pkcs12_attributes"}
// Dependencies: {}
fn pkcs12_attributes < 'a > (friendly_name : Option < & 'a [u8] > , local_key_id : Option < & 'a [u8] > , is_java_trusted_cert : bool ,) -> CryptographyResult < Option < asn1 :: SetOfWriter < 'a , cryptography_x509 :: pkcs12 :: Attribute < 'a > , Vec < cryptography_x509 :: pkcs12 :: Attribute < 'a > > , > , > , > { let mut attrs = vec ! [] ; if let Some (name) = friendly_name { let name_str = std :: str :: from_utf8 (name) . map_err (| _ | { pyo3 :: exceptions :: PyValueError :: new_err ("friendly_name must be valid UTF-8") }) ? ; attrs . push (cryptography_x509 :: pkcs12 :: Attribute { _attr_id : asn1 :: DefinedByMarker :: marker () , attr_values : cryptography_x509 :: pkcs12 :: AttributeSet :: FriendlyName (asn1 :: SetOfWriter :: new ([Utf8StoredBMPString :: new (name_str)]) ,) , }) ; } if let Some (key_id) = local_key_id { attrs . push (cryptography_x509 :: pkcs12 :: Attribute { _attr_id : asn1 :: DefinedByMarker :: marker () , attr_values : cryptography_x509 :: pkcs12 :: AttributeSet :: LocalKeyId (asn1 :: SetOfWriter :: new ([key_id]) ,) , }) ; } if is_java_trusted_cert { attrs . push (cryptography_x509 :: pkcs12 :: Attribute { _attr_id : asn1 :: DefinedByMarker :: marker () , attr_values : cryptography_x509 :: pkcs12 :: AttributeSet :: JDKTruststoreUsage (asn1 :: SetOfWriter :: new ([EKU_ANY_KEY_USAGE_OID]) ,) , }) ; } if attrs . is_empty () { Ok (None) } else { Ok (Some (asn1 :: SetOfWriter :: new (attrs))) } }
};
}
