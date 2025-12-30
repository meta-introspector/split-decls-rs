// Generated macro for Attribute (struct)
macro_rules! Depcrate_pkcs12Attribute {
() => {
// Module: crate::pkcs12
// Provides: {"Attribute"}
// Dependencies: {}
# [derive (asn1 :: Asn1Write)] pub struct Attribute < 'a > { pub _attr_id : asn1 :: DefinedByMarker < asn1 :: ObjectIdentifier > , # [defined_by (_attr_id)] pub attr_values : AttributeSet < 'a > , }
};
}
