// Generated macro for impl_71 (impl)
macro_rules! Depcrate_builtin_typeimpl_71 {
() => {
// Module: crate::builtin_type
// Provides: {"impl_71"}
// Dependencies: {}
# [rustfmt :: skip] impl BuiltinUint { pub fn from_suffix (suffix : & str) -> Option < BuiltinUint > { let res = match suffix { "usize" => Self :: Usize , "u8" => Self :: U8 , "u16" => Self :: U16 , "u32" => Self :: U32 , "u64" => Self :: U64 , "u128" => Self :: U128 , _ => return None , } ; Some (res) } pub fn from_suffix_sym (suffix : & Symbol) -> Option < BuiltinUint > { let res = match suffix { s if * s == sym :: usize => Self :: Usize , s if * s == sym :: u8 => Self :: U8 , s if * s == sym :: u16 => Self :: U16 , s if * s == sym :: u32 => Self :: U32 , s if * s == sym :: u64 => Self :: U64 , s if * s == sym :: u128 => Self :: U128 , _ => return None , } ; Some (res) } }
};
}
