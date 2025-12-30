// Generated macro for impl_46 (impl)
macro_rules! Depcrate_sharedimpl_46 {
() => {
// Module: crate::shared
// Provides: {"impl_46"}
// Dependencies: {}
impl fmt :: Display for FloatTy { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let s = match self { FloatTy :: F16 => "f16" , FloatTy :: F32 => "f32" , FloatTy :: F64 => "f64" , FloatTy :: F128 => "f128" , } ; f . write_str (s) } }
};
}
