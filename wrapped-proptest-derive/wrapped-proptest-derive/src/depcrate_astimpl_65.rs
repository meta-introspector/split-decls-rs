// Generated macro for impl_65 (impl)
macro_rules! Depcrate_astimpl_65 {
() => {
// Module: crate::ast
// Provides: {"impl_65"}
// Dependencies: {}
impl ToTokens for MapClosure { fn to_tokens (& self , tokens : & mut TokenStream) { fn tmp_var < 'a > (idx : usize) -> FreshVar < 'a > { fresh_var ("tmp" , idx) } let MapClosure (path , fields) = self ; let count = fields . len () ; let tmps : Vec < _ > = (0 .. count) . map (tmp_var) . collect () ; let inits = fields . iter () . enumerate () . map (| (idx , field) | { let tv = tmp_var (idx) ; if let Some (name) = & field . ident { quote_spanned ! (field . span () => # name : # tv) } else { let name = syn :: Member :: Unnamed (syn :: Index :: from (idx)) ; quote_spanned ! (field . span () => # name : # tv) } }) ; let tmps = NestedTuple (& tmps) ; quote_append ! (tokens , | # tmps | # path { # (# inits) ,* }) ; } }
};
}
