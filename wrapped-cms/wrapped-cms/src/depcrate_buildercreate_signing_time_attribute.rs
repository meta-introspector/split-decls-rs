// Generated macro for create_signing_time_attribute (function)
macro_rules! Depcrate_buildercreate_signing_time_attribute {
() => {
// Module: crate::builder
// Provides: {"create_signing_time_attribute"}
// Dependencies: {}
# [doc = " Create a signing time attribute according to"] # [doc = " [RFC 5652 § 11.3](https://datatracker.ietf.org/doc/html/rfc5652#section-11.3)"] # [doc = " Dates between 1 January 1950 and 31 December 2049 (inclusive) MUST be"] # [doc = " encoded as UTCTime.  Any dates with year values before 1950 or after"] # [doc = " 2049 MUST be encoded as GeneralizedTime."] pub fn create_signing_time_attribute () -> Result < Attribute > { let time_der = Time :: now () ? . to_der () ? ; let signing_time_attribute_value = AttributeValue :: from_der (& time_der) ? ; let mut values = SetOfVec :: < AttributeValue > :: new () ; values . insert (signing_time_attribute_value) ? ; let attribute = Attribute { oid : const_oid :: db :: rfc5911 :: ID_SIGNING_TIME , values , } ; Ok (attribute) }
};
}
