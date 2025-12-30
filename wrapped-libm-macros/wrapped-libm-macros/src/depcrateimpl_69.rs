// Generated macro for impl_69 (impl)
macro_rules! Depcrateimpl_69 {
() => {
// Module: crate
// Provides: {"impl_69"}
// Dependencies: {}
impl ToTokens for FloatTy { fn to_tokens (& self , tokens : & mut pm2 :: TokenStream) { let ts = match self { FloatTy :: F16 => quote ! { f16 } , FloatTy :: F32 => quote ! { f32 } , FloatTy :: F64 => quote ! { f64 } , FloatTy :: F128 => quote ! { f128 } , } ; tokens . extend (ts) ; } }
};
}
