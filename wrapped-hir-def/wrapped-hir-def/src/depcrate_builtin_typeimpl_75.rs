// Generated macro for impl_75 (impl)
macro_rules! Depcrate_builtin_typeimpl_75 {
() => {
// Module: crate::builtin_type
// Provides: {"impl_75"}
// Dependencies: {}
impl fmt :: Display for BuiltinFloat { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { BuiltinFloat :: F16 => "f16" , BuiltinFloat :: F32 => "f32" , BuiltinFloat :: F64 => "f64" , BuiltinFloat :: F128 => "f128" , }) } }
};
}
