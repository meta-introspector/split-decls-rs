// Generated macro for other_52 (other)
macro_rules! Depcrate_utilsother_52 {
() => {
// Module: crate::utils
// Provides: {"other_52"}
// Dependencies: {}
# [allow (dead_code)] # [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "powerpc64" , target_arch = "riscv64" , target_arch = "s390x" , target_arch = "x86_64" ,))] # [doc = " A 128-bit value represented as a pair of 64-bit values."] # [doc = ""] # [doc = " This type is `#[repr(C)]`, both fields have the same in-memory representation"] # [doc = " and are plain old data types, so access to the fields is always safe."] # [derive (Clone , Copy)] # [repr (C)] pub (crate) union U128 { pub (crate) whole : u128 , pub (crate) pair : Pair < u64 > , }
};
}
