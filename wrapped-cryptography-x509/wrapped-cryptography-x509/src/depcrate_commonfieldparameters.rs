// Generated macro for FieldParameters (enum)
macro_rules! Depcrate_commonFieldParameters {
() => {
// Module: crate::common
// Provides: {"FieldParameters"}
// Dependencies: {}
# [derive (asn1 :: Asn1DefinedByRead , asn1 :: Asn1DefinedByWrite , Hash , Clone , PartialEq , Eq , Debug)] pub enum FieldParameters < 'a > { # [defined_by (oid :: PRIME_FIELD_OID)] PrimeField (asn1 :: BigUint < 'a >) , # [defined_by (oid :: CHARACTERISTIC_TWO_FIELD_OID)] CharacteristicTwo (asn1 :: Sequence < 'a >) , }
};
}
