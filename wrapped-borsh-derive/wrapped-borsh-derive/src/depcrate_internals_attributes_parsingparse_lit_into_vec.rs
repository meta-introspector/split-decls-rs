// Generated macro for parse_lit_into_vec (function)
macro_rules! Depcrate_internals_attributes_parsingparse_lit_into_vec {
() => {
// Module: crate::internals::attributes::parsing
// Provides: {"parse_lit_into_vec"}
// Dependencies: {}
pub (super) fn parse_lit_into_vec < T : syn :: parse :: Parse > (attr_name : Symbol , meta_item_name : Symbol , meta : & ParseNestedMeta ,) -> syn :: Result < Vec < T > > { let string = get_lit_str2 (attr_name , meta_item_name , meta) ? ; match string . parse_with (Punctuated :: < T , Token ! [,] > :: parse_terminated) { Ok (elements) => Ok (Vec :: from_iter (elements)) , Err (err) => Err (syn :: Error :: new_spanned (string , err)) , } }
};
}
