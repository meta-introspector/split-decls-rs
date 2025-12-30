// Generated macro for impl_35 (impl)
macro_rules! Depcrate_fuzzersimpl_35 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_35"}
// Dependencies: {}
# [doc = " A qucickcheck trait for describing how `TypeQualifierC` types can be"] # [doc = " randomly generated and shrunk."] impl Arbitrary for TypeQualifierC { fn arbitrary (g : & mut Gen) -> TypeQualifierC { let qualifier = vec ! ["const" , ""] ; TypeQualifierC { def : String :: from (* g . choose (& qualifier) . unwrap ()) , } } }
};
}
