// Generated macro for impl_31 (impl)
macro_rules! Depcrate_fuzzersimpl_31 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_31"}
// Dependencies: {}
# [doc = " A qucickcheck trait for describing how `DeclarationListC` types can be"] # [doc = " randomly generated and shrunk."] impl Arbitrary for DeclarationListC { fn arbitrary (g : & mut Gen) -> DeclarationListC { DeclarationListC { decls : Arbitrary :: arbitrary (g) , } } }
};
}
