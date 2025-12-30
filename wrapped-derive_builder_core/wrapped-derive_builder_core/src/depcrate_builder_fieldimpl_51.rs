// Generated macro for impl_51 (impl)
macro_rules! Depcrate_builder_fieldimpl_51 {
() => {
// Module: crate::builder_field
// Provides: {"impl_51"}
// Dependencies: {}
impl < 'a > ToTokens for BuilderFieldTypeWithCrateRoot < 'a > { fn to_tokens (& self , tokens : & mut TokenStream) { let crate_root = self . crate_root ; match self . field_type { BuilderFieldType :: Optional (ty) => tokens . append_all (quote ! (# crate_root :: export :: core :: option :: Option <# ty >)) , BuilderFieldType :: Precise (ty) => ty . to_tokens (tokens) , BuilderFieldType :: Phantom (ty) => tokens . append_all (quote ! (# crate_root :: export :: core :: marker :: PhantomData <# ty >)) , } } }
};
}
