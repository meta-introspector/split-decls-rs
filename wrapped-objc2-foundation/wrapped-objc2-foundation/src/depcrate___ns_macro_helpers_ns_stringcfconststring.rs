// Generated macro for CFConstString (struct)
macro_rules! Depcrate___ns_macro_helpers_ns_stringCFConstString {
() => {
// Module: crate::__ns_macro_helpers::ns_string
// Provides: {"CFConstString"}
// Dependencies: {}
# [doc = " Structure used to describe a constant `CFString`."] # [doc = ""] # [doc = " This struct is the same as [`CF_CONST_STRING`], which contains"] # [doc = " [`CFRuntimeBase`]. While the documentation clearly says that the ABI of"] # [doc = " `CFRuntimeBase` should not be relied on, we can rely on it as long as we"] # [doc = " only do it with regards to `CFString` (because `clang` does this as well)."] # [doc = ""] # [doc = " [`CFRuntimeBase`]: <https://github.com/apple-oss-distributions/CF/blob/CF-1153.18/CFRuntime.h#L216-L228>"] # [doc = " [`CF_CONST_STRING`]: <https://github.com/apple-oss-distributions/CF/blob/CF-1153.18/CFInternal.h#L332-L336>"] # [repr (C)] # [derive (Debug)] pub struct CFConstString { isa : & 'static AnyClass , cfinfo : u32 , # [cfg (target_pointer_width = "64")] _rc : u32 , data : * const c_void , len : usize , }
};
}
