// Generated macro for impl_70 (impl)
macro_rules! Depcrate_builtin_typeimpl_70 {
() => {
// Module: crate::builtin_type
// Provides: {"impl_70"}
// Dependencies: {}
# [rustfmt :: skip] impl BuiltinInt { pub fn from_suffix (suffix : & str) -> Option < BuiltinInt > { let res = match suffix { "isize" => Self :: Isize , "i8" => Self :: I8 , "i16" => Self :: I16 , "i32" => Self :: I32 , "i64" => Self :: I64 , "i128" => Self :: I128 , _ => return None , } ; Some (res) } pub fn from_suffix_sym (suffix : & Symbol) -> Option < BuiltinInt > { let res = match suffix { s if * s == sym :: isize => Self :: Isize , s if * s == sym :: i8 => Self :: I8 , s if * s == sym :: i16 => Self :: I16 , s if * s == sym :: i32 => Self :: I32 , s if * s == sym :: i64 => Self :: I64 , s if * s == sym :: i128 => Self :: I128 , _ => return None , } ; Some (res) } }
};
}
