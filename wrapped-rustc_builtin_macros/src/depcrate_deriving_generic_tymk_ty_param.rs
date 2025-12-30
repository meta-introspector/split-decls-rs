// Generated macro for mk_ty_param (function)
macro_rules! Depcrate_deriving_generic_tymk_ty_param {
() => {
// Module: crate::deriving::generic::ty
// Provides: {"mk_ty_param"}
// Dependencies: {}
fn mk_ty_param (cx : & ExtCtxt < '_ > , span : Span , name : Symbol , bounds : & [Path] , self_ident : Ident , self_generics : & Generics ,) -> ast :: GenericParam { let bounds = bounds . iter () . map (| b | { let path = b . to_path (cx , span , self_ident , self_generics) ; cx . trait_bound (path , false) }) . collect () ; cx . typaram (span , Ident :: new (name , span) , bounds , None) }
};
}
