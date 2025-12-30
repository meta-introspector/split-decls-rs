// Generated macro for parse_meta_name_value_after_path (function)
macro_rules! Depcrate_ast_data_nested_metaparse_meta_name_value_after_path {
() => {
// Module: crate::ast::data::nested_meta
// Provides: {"parse_meta_name_value_after_path"}
// Dependencies: {}
fn parse_meta_name_value_after_path < 'a > (path : Path , input : ParseStream < 'a > ,) -> syn :: Result < MetaNameValue > { let eq_token : Token ! [=] = input . parse () ? ; let ahead = input . fork () ; let lit : Option < Lit > = ahead . parse () ? ; let value = if let (Some (lit) , true) = (lit , ahead . is_empty ()) { input . advance_to (& ahead) ; Expr :: Lit (ExprLit { attrs : Vec :: new () , lit , }) } else if input . peek (Token ! [#]) && input . peek2 (token :: Bracket) { return Err (input . error ("unexpected attribute inside of attribute")) ; } else { input . parse () ? } ; Ok (MetaNameValue { path , eq_token , value , }) }
};
}
