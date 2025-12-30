// Generated macro for impl_76 (impl)
macro_rules! Depcrate_builtin_typeimpl_76 {
() => {
// Module: crate::builtin_type
// Provides: {"impl_76"}
// Dependencies: {}
impl fmt :: Display for BuiltinFloat { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { BuiltinFloat :: F16 => "f16" , BuiltinFloat :: F32 => "f32" , BuiltinFloat :: F64 => "f64" , BuiltinFloat :: F128 => "f128" , }) } }
};
}
