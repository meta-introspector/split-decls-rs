// Generated macro for impl_54 (impl)
macro_rules! Depcrate_fuzzersimpl_54 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_54"}
// Dependencies: {}
# [doc = " A qucickcheck trait for describing how `FunctionPrototypeC` types can be"] # [doc = " randomly generated and shrunk."] impl Arbitrary for FunctionPrototypeC { fn arbitrary (g : & mut Gen) -> FunctionPrototypeC { FunctionPrototypeC { type_qualifier : Arbitrary :: arbitrary (g) , type_name : Arbitrary :: arbitrary (g) , pointer_level : Arbitrary :: arbitrary (g) , params : Arbitrary :: arbitrary (g) , ident_id : format ! ("{}" , usize :: arbitrary (g)) , } } }
};
}
