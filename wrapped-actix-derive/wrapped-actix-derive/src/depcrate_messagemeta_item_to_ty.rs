// Generated macro for meta_item_to_ty (function)
macro_rules! Depcrate_messagemeta_item_to_ty {
() => {
// Module: crate::message
// Provides: {"meta_item_to_ty"}
// Dependencies: {}
fn meta_item_to_ty (meta_item : & syn :: Meta) -> syn :: Result < syn :: Type > { match meta_item { syn :: Meta :: Path (path) => match path . get_ident () { Some (ident) => syn :: parse_str :: < syn :: Type > (& ident . to_string ()) . map_err (| _ | syn :: Error :: new_spanned (ident , "Expect type")) , None => Err (syn :: Error :: new_spanned (path , "Expect type")) , } , syn :: Meta :: NameValue (nv) => match nv . path . get_ident () { Some (ident) if ident == "result" => { if let syn :: Expr :: Lit (syn :: ExprLit { lit : syn :: Lit :: Str (lit) , .. }) = nv . value . clone () { if let Ok (ty) = syn :: parse_str :: < syn :: Type > (& lit . value ()) { return Ok (ty) ; } } Err (syn :: Error :: new_spanned (& nv . value , "Expect type")) } _ => Err (syn :: Error :: new_spanned (& nv . value , r#"Expect `result = "TYPE"`"# ,)) , } , syn :: Meta :: List (list) => { let lit_str = syn :: parse2 :: < syn :: LitStr > (list . tokens . clone ()) . map_err (| _ | syn :: Error :: new_spanned (list , "Expect type")) ? ; syn :: parse_str (& lit_str . value ()) . map_err (| _ | syn :: Error :: new_spanned (list , "Expect type")) } } }
};
}
