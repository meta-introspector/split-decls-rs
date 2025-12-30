// Generated macro for Integer (struct)
macro_rules! Depcrate_value_integerInteger {
() => {
// Module: crate::value::integer
// Provides: {"Integer"}
// Dependencies: {}
# [doc = " An abstract integer value"] # [doc = ""] # [doc = " This opaque type represents an integer value which can be encoded in CBOR"] # [doc = " without resulting to big integer encoding. Larger values may be encoded"] # [doc = " using the big integer encoding as described in the CBOR RFC. See the"] # [doc = " implementations for 128-bit integer conversions on `Value` for more"] # [doc = " details."] # [derive (Copy , Clone , Debug , Hash , PartialEq , Eq , PartialOrd , Ord)] pub struct Integer (i128) ;
};
}
