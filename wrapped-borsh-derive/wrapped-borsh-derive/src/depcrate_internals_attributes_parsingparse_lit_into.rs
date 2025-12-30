// Generated macro for parse_lit_into (function)
macro_rules! Depcrate_internals_attributes_parsingparse_lit_into {
() => {
// Module: crate::internals::attributes::parsing
// Provides: {"parse_lit_into"}
// Dependencies: {}
pub (super) fn parse_lit_into < T : syn :: parse :: Parse > (attr_name : Symbol , meta_item_name : Symbol , meta : & ParseNestedMeta ,) -> syn :: Result < T > { let string = get_lit_str2 (attr_name , meta_item_name , meta) ? ; match string . parse () { Ok (expr) => Ok (expr) , Err (err) => Err (syn :: Error :: new_spanned (string , err)) , } }
};
}
