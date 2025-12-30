// Generated macro for other_27 (other)
macro_rules! Depcrate_utilsother_27 {
() => {
// Module: crate::utils
// Provides: {"other_27"}
// Dependencies: {}
# [doc = " A 64-bit value represented as a pair of 32-bit values."] # [doc = ""] # [doc = " This type is `#[repr(C)]`, both fields have the same in-memory representation"] # [doc = " and are plain old data types, so access to the fields is always safe."] # [allow (dead_code)] # [derive (Clone , Copy)] # [repr (C)] pub (crate) union MaybeUninit64 { pub (crate) whole : MaybeUninit < u64 > , pub (crate) pair : Pair < u32 > , }
};
}
