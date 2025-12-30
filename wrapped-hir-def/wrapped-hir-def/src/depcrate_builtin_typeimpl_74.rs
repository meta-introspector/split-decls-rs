// Generated macro for impl_74 (impl)
macro_rules! Depcrate_builtin_typeimpl_74 {
() => {
// Module: crate::builtin_type
// Provides: {"impl_74"}
// Dependencies: {}
impl fmt :: Display for BuiltinUint { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { BuiltinUint :: Usize => "usize" , BuiltinUint :: U8 => "u8" , BuiltinUint :: U16 => "u16" , BuiltinUint :: U32 => "u32" , BuiltinUint :: U64 => "u64" , BuiltinUint :: U128 => "u128" , }) } }
};
}
