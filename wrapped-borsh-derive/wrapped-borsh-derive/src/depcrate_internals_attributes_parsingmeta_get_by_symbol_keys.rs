// Generated macro for meta_get_by_symbol_keys (function)
macro_rules! Depcrate_internals_attributes_parsingmeta_get_by_symbol_keys {
() => {
// Module: crate::internals::attributes::parsing
// Provides: {"meta_get_by_symbol_keys"}
// Dependencies: {}
pub (super) fn meta_get_by_symbol_keys < T , F > (attr_name : Symbol , meta : & ParseNestedMeta , map : & BTreeMap < Symbol , F > ,) -> syn :: Result < BTreeMap < Symbol , T > > where F : Fn (Symbol , Symbol , & ParseNestedMeta) -> syn :: Result < T > , { let mut result = BTreeMap :: new () ; let lookahead = meta . input . lookahead1 () ; if lookahead . peek (Paren) { meta . parse_nested_meta (| meta | get_nested_meta_logic (attr_name , meta , map , & mut result)) ? ; } else { return Err (lookahead . error ()) ; } Ok (result) }
};
}
