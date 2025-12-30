// Generated macro for impl_45 (impl)
macro_rules! Depcrate_fuzzersimpl_45 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_45"}
// Dependencies: {}
# [doc = " A qucickcheck trait for describing how `StructDeclarationC` types can be"] # [doc = " randomly generated and shrunk."] impl Arbitrary for StructDeclarationC { fn arbitrary (g : & mut Gen) -> StructDeclarationC { let reduced_size : usize = (g . size () / 2) + 1 ; let mut decl_list : DeclarationListC = Arbitrary :: arbitrary (& mut Gen :: new (reduced_size)) ; let mut fields : DeclarationListC = DeclarationListC { decls : vec ! [] } ; for (i , decl) in decl_list . decls . iter_mut () . enumerate () { match * decl { DeclarationC :: FunctionDecl (_) => { } ref mut decl => { decl . make_unique (i) ; fields . decls . push (decl . clone ()) ; } } } StructDeclarationC { fields , ident_id : format ! ("{}" , usize :: arbitrary (g)) , array_dimension : Arbitrary :: arbitrary (g) , } } }
};
}
