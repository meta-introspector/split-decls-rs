// Generated macro for get_lit_str2 (function)
macro_rules! Depcrate_internals_attributes_parsingget_lit_str2 {
() => {
// Module: crate::internals::attributes::parsing
// Provides: {"get_lit_str2"}
// Dependencies: {}
fn get_lit_str2 (attr_name : Symbol , meta_item_name : Symbol , meta : & ParseNestedMeta ,) -> syn :: Result < LitStr > { let expr : Expr = meta . value () ? . parse () ? ; let mut value = & expr ; while let Expr :: Group (e) = value { value = & e . expr ; } if let Expr :: Lit (syn :: ExprLit { lit : Lit :: Str (lit) , .. }) = value { Ok (lit . clone ()) } else { Err (syn :: Error :: new_spanned (expr , format ! ("expected borsh {} attribute to be a string: `{} = \"...\"`" , attr_name . 0 , meta_item_name . 0) ,)) } }
};
}
