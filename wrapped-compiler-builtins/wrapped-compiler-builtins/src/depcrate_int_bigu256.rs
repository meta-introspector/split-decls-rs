// Generated macro for u256 (struct)
macro_rules! Depcrate_int_bigu256 {
() => {
// Module: crate::int::big
// Provides: {"u256"}
// Dependencies: {}
# [doc = " A 256-bit unsigned integer represented as 4 64-bit limbs."] # [doc = ""] # [doc = " Each limb is a native-endian number, but the array is little-limb-endian."] # [allow (non_camel_case_types)] # [derive (Clone , Copy , Debug , PartialEq , PartialOrd)] pub struct u256 (pub [u64 ; 4]) ;
};
}
