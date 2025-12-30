// Generated macro for impl_56 (impl)
macro_rules! Depcrate_fuzzersimpl_56 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_56"}
// Dependencies: {}
# [doc = " A qucickcheck trait for describing how `ParameterC` types can be"] # [doc = " randomly generated and shrunk."] impl Arbitrary for ParameterC { fn arbitrary (g : & mut Gen) -> ParameterC { ParameterC { type_qualifier : Arbitrary :: arbitrary (g) , type_name : Arbitrary :: arbitrary (g) , pointer_level : Arbitrary :: arbitrary (g) , } } }
};
}
