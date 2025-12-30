// Generated macro for i256 (struct)
macro_rules! Depcrate_int_bigi256 {
() => {
// Module: crate::int::big
// Provides: {"i256"}
// Dependencies: {}
# [doc = " A 256-bit signed integer represented as 4 64-bit limbs."] # [doc = ""] # [doc = " Each limb is a native-endian number, but the array is little-limb-endian."] # [allow (non_camel_case_types)] # [derive (Clone , Copy , Debug , PartialEq , PartialOrd)] pub struct i256 (pub [u64 ; 4]) ;
};
}
