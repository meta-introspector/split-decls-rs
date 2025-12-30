// Generated macro for impl_58 (impl)
macro_rules! Depcrate_fuzzersimpl_58 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_58"}
// Dependencies: {}
# [doc = " A qucickcheck trait for describing how `ParameterListC` types can be"] # [doc = " randomly generated and shrunk."] impl Arbitrary for ParameterListC { fn arbitrary (g : & mut Gen) -> ParameterListC { ParameterListC { params : Arbitrary :: arbitrary (g) , } } }
};
}
