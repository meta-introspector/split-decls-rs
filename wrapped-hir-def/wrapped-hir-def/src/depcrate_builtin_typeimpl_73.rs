// Generated macro for impl_73 (impl)
macro_rules! Depcrate_builtin_typeimpl_73 {
() => {
// Module: crate::builtin_type
// Provides: {"impl_73"}
// Dependencies: {}
impl fmt :: Display for BuiltinInt { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { BuiltinInt :: Isize => "isize" , BuiltinInt :: I8 => "i8" , BuiltinInt :: I16 => "i16" , BuiltinInt :: I32 => "i32" , BuiltinInt :: I64 => "i64" , BuiltinInt :: I128 => "i128" , }) } }
};
}
