// Generated macro for FieldID (struct)
macro_rules! Depcrate_commonFieldID {
() => {
// Module: crate::common
// Provides: {"FieldID"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write , Hash , Clone , PartialEq , Eq , Debug)] pub struct FieldID < 'a > { pub field_type : asn1 :: DefinedByMarker < asn1 :: ObjectIdentifier > , # [defined_by (field_type)] pub parameters : FieldParameters < 'a > , }
};
}
