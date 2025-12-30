// Generated macro for build_debug_expr (function)
macro_rules! Depcrate_item_typebuild_debug_expr {
() => {
// Module: crate::item_type
// Provides: {"build_debug_expr"}
// Dependencies: {}
fn build_debug_expr (ident : & Ident , fields_source : & Fields , fields : & [FieldEntry] , use_bounds : bool , to_expr : impl Fn (& FieldEntry) -> TokenStream , wcb : & mut WhereClauseBuilder ,) -> Result < TokenStream > { let kind = DeriveItemKind :: Debug ; let mut transparent_field = None ; for field in fields { if let Some (span) = field . hattrs . debug . transparent . span { if transparent_field . is_some () { bail ! (span , "only one field can be set `#[debug(transparent)]`") ; } transparent_field = Some (field) ; } } let expr = if let Some (field) = transparent_field { let e = to_expr (field) ; field . push_bounds_to (use_bounds , kind , wcb) ; quote_spanned ! (field . span () => :: core :: fmt :: Debug :: fmt (# e , f)) } else { let is_named = match fields_source { Fields :: Named (_) => true , Fields :: Unnamed (_) | Fields :: Unit => false , } ; let mut expr = TokenStream :: new () ; let debug_x = match is_named { true => quote ! (debug_struct) , false => quote ! (debug_tuple) , } ; expr . extend (quote ! (f .# debug_x (:: core :: stringify ! (# ident)))) ; for field in fields { if ! field . hattrs . is_debug_skip () { let e = to_expr (field) ; let member = field . member () ; let span = field . span () ; expr . extend (match is_named { true => quote_spanned ! (span => . field (:: core :: stringify ! (# member) , # e)) , false => quote_spanned ! (span => . field (# e)) , }) ; field . push_bounds_to (use_bounds , kind , wcb) ; } } expr . extend (quote ! (. finish ())) ; expr } ; Ok (expr) }
};
}
