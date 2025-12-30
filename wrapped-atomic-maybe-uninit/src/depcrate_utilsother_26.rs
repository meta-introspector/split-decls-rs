// Generated macro for other_26 (other)
macro_rules! Depcrate_utilsother_26 {
() => {
// Module: crate::utils
// Provides: {"other_26"}
// Dependencies: {}
# [doc = " A 128-bit value represented as a pair of 64-bit values."] # [doc = ""] # [doc = " This type is `#[repr(C)]`, both fields have the same in-memory representation"] # [doc = " and are plain old data types, so access to the fields is always safe."] # [allow (dead_code)] # [derive (Clone , Copy)] # [repr (C)] pub (crate) union MaybeUninit128 { pub (crate) whole : MaybeUninit < u128 > , pub (crate) pair : Pair < u64 > , }
};
}
