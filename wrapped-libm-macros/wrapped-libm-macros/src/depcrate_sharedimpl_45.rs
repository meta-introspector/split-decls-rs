// Generated macro for impl_45 (impl)
macro_rules! Depcrate_sharedimpl_45 {
() => {
// Module: crate::shared
// Provides: {"impl_45"}
// Dependencies: {}
impl fmt :: Display for Ty { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let s = match self { Ty :: F16 => "f16" , Ty :: F32 => "f32" , Ty :: F64 => "f64" , Ty :: F128 => "f128" , Ty :: I32 => "i32" , Ty :: CInt => "::core::ffi::c_int" , Ty :: MutF16 => "&mut f16" , Ty :: MutF32 => "&mut f32" , Ty :: MutF64 => "&mut f64" , Ty :: MutF128 => "&mut f128" , Ty :: MutI32 => "&mut i32" , Ty :: MutCInt => "&mut ::core::ffi::c_int" , } ; f . write_str (s) } }
};
}
