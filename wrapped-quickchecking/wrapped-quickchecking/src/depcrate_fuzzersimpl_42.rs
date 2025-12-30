// Generated macro for impl_42 (impl)
macro_rules! Depcrate_fuzzersimpl_42 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_42"}
// Dependencies: {}
# [doc = " A qucickcheck trait for describing how `BasicTypeDeclarationC` types can be"] # [doc = " randomly generated and shrunk."] impl Arbitrary for BasicTypeDeclarationC { fn arbitrary (g : & mut Gen) -> BasicTypeDeclarationC { BasicTypeDeclarationC { type_qualifier : Arbitrary :: arbitrary (g) , type_name : Arbitrary :: arbitrary (g) , pointer_level : Arbitrary :: arbitrary (g) , array_dimension : Arbitrary :: arbitrary (g) , ident_id : format ! ("{}" , usize :: arbitrary (g)) , } } }
};
}
