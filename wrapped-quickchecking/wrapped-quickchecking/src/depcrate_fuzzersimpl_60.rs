// Generated macro for impl_60 (impl)
macro_rules! Depcrate_fuzzersimpl_60 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_60"}
// Dependencies: {}
# [doc = " A qucickcheck trait for describing how `HeaderC` types can be"] # [doc = " randomly generated and shrunk."] impl Arbitrary for HeaderC { fn arbitrary (g : & mut Gen) -> HeaderC { let mut decl_list : DeclarationListC = Arbitrary :: arbitrary (g) ; for (i , decl) in decl_list . decls . iter_mut () . enumerate () { decl . make_unique (i) ; } HeaderC { def : decl_list } } }
};
}
