// Generated macro for tit2iit (function)
macro_rules! Depcrate_mockable_structtit2iit {
() => {
// Module: crate::mockable_struct
// Provides: {"tit2iit"}
// Dependencies: {}
# [doc = " Converts a TraitItemType into an ImplItemType"] fn tit2iit (tit : TraitItemType , vis : & Visibility) -> ImplItemType { let span = tit . span () ; let (eq_token , ty) = tit . default . unwrap_or_else (| | { compile_error (span , "associated types in mock! must be fully specified") ; (token :: Eq :: default () , Type :: Verbatim (TokenStream :: new ())) }) ; ImplItemType { attrs : tit . attrs , vis : vis . clone () , defaultness : None , type_token : tit . type_token , ident : tit . ident , generics : tit . generics , eq_token , ty , semi_token : tit . semi_token , } }
};
}
