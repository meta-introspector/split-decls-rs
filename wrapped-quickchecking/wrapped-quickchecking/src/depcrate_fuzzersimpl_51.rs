// Generated macro for impl_51 (impl)
macro_rules! Depcrate_fuzzersimpl_51 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_51"}
// Dependencies: {}
# [doc = " A qucickcheck trait for describing how `FunctionPointerDeclarationC` types can"] # [doc = " be randomly generated and shrunk."] impl Arbitrary for FunctionPointerDeclarationC { fn arbitrary (g : & mut Gen) -> FunctionPointerDeclarationC { FunctionPointerDeclarationC { type_qualifier : Arbitrary :: arbitrary (g) , type_name : Arbitrary :: arbitrary (g) , pointer_level : Arbitrary :: arbitrary (g) , params : Arbitrary :: arbitrary (g) , ident_id : format ! ("{}" , usize :: arbitrary (g)) , } } }
};
}
