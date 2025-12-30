// Generated macro for other_53 (other)
macro_rules! Depcrate_utilsother_53 {
() => {
// Module: crate::utils
// Provides: {"other_53"}
// Dependencies: {}
# [allow (dead_code)] # [cfg (any (target_arch = "arm" , target_arch = "riscv32"))] # [doc = " A 64-bit value represented as a pair of 32-bit values."] # [doc = ""] # [doc = " This type is `#[repr(C)]`, both fields have the same in-memory representation"] # [doc = " and are plain old data types, so access to the fields is always safe."] # [derive (Clone , Copy)] # [repr (C)] pub (crate) union U64 { pub (crate) whole : u64 , pub (crate) pair : Pair < u32 > , }
};
}
