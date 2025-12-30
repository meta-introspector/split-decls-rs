// Generated macro for impl_117 (impl)
macro_rules! Depcrate_codegen_fieldimpl_117 {
() => {
// Module: crate::codegen::field
// Provides: {"impl_117"}
// Dependencies: {}
impl ToTokens for Declaration < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { let field = self . 0 ; let ident = field . ident ; let ty = field . ty ; tokens . append_all (if field . multiple { quote ! (let mut # ident : # ty = :: darling :: export :: Default :: default () ;) } else { quote ! (let mut # ident : (bool , :: darling :: export :: Option <# ty >) = (false , None) ;) }) ; if field . flatten { tokens . append_all (quote ! { let mut __flatten : Vec <:: darling :: ast :: NestedMeta > = vec ! [] ; }) ; } } }
};
}
