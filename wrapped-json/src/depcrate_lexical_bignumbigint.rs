// Generated macro for Bigint (struct)
macro_rules! Depcrate_lexical_bignumBigint {
() => {
// Module: crate::lexical::bignum
// Provides: {"Bigint"}
// Dependencies: {}
# [doc = " Storage for a big integer type."] # [derive (Clone , PartialEq , Eq)] pub (crate) struct Bigint { # [doc = " Internal storage for the Bigint, in little-endian order."] pub (crate) data : Vec < Limb > , }
};
}
