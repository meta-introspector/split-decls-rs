// Generated macro for impl_68 (impl)
macro_rules! Depcrateimpl_68 {
() => {
// Module: crate
// Provides: {"impl_68"}
// Dependencies: {}
impl ToTokens for Ty { fn to_tokens (& self , tokens : & mut pm2 :: TokenStream) { let ts = match self { Ty :: F16 => quote ! { f16 } , Ty :: F32 => quote ! { f32 } , Ty :: F64 => quote ! { f64 } , Ty :: F128 => quote ! { f128 } , Ty :: I32 => quote ! { i32 } , Ty :: CInt => quote ! { :: core :: ffi :: c_int } , Ty :: MutF16 => quote ! { &'a mut f16 } , Ty :: MutF32 => quote ! { &'a mut f32 } , Ty :: MutF64 => quote ! { &'a mut f64 } , Ty :: MutF128 => quote ! { &'a mut f128 } , Ty :: MutI32 => quote ! { &'a mut i32 } , Ty :: MutCInt => quote ! { &'a mut core :: ffi :: c_int } , } ; tokens . extend (ts) ; } }
};
}
