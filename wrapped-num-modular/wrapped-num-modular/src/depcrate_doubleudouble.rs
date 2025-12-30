// Generated macro for udouble (struct)
macro_rules! Depcrate_doubleudouble {
() => {
// Module: crate::double
// Provides: {"udouble"}
// Dependencies: {}
# [allow (non_camel_case_types)] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord)] # [doc = " A double width integer type based on the largest built-in integer type [umax] (currently [u128]), and"] # [doc = " to support double-width operations on it is the only goal for this type."] # [doc = ""] # [doc = " Although it can be regarded as u256, it's not as feature-rich as in other crates"] # [doc = " since it's only designed to support this crate and few other crates (will be noted in comments)."] pub struct udouble { # [doc = " Most significant part"] pub hi : umax , # [doc = " Least significant part"] pub lo : umax , }
};
}
