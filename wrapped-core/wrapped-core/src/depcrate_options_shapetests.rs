// Generated macro for tests (module)
macro_rules! Depcrate_options_shapetests {
() => {
// Module: crate::options::shape
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use proc_macro2 :: TokenStream ; use quote :: quote ; use syn :: parse_quote ; use super :: DeriveInputShapeSet ; use crate :: FromMeta ; # [doc = " parse a string as a syn::Meta instance."] fn pm (tokens : TokenStream) -> :: std :: result :: Result < syn :: Meta , String > { let attribute : syn :: Attribute = parse_quote ! (# [# tokens]) ; Ok (attribute . meta) } fn fm < T : FromMeta > (tokens : TokenStream) -> T { FromMeta :: from_meta (& pm (tokens) . expect ("Tests should pass well-formed input")) . expect ("Tests should pass valid input") } # [test] fn supports_any () { let decl = fm :: < DeriveInputShapeSet > (quote ! (ignore (any))) ; assert ! (decl . any) ; } # [test] fn supports_struct () { let decl = fm :: < DeriveInputShapeSet > (quote ! (ignore (struct_any , struct_newtype))) ; assert ! (decl . struct_values . any) ; assert ! (decl . struct_values . newtype) ; } # [test] fn supports_mixed () { let decl = fm :: < DeriveInputShapeSet > (quote ! (ignore (struct_newtype , enum_newtype , enum_tuple))) ; assert ! (decl . struct_values . newtype) ; assert ! (decl . enum_values . newtype) ; assert ! (decl . enum_values . tuple) ; assert ! (! decl . struct_values . any) ; } }
};
}
