// Generated macro for attr_get_by_symbol_keys (function)
macro_rules! Depcrate_internals_attributes_parsingattr_get_by_symbol_keys {
() => {
// Module: crate::internals::attributes::parsing
// Provides: {"attr_get_by_symbol_keys"}
// Dependencies: {}
pub (super) fn attr_get_by_symbol_keys < T , F > (attr_name : Symbol , attr : & Attribute , map : & BTreeMap < Symbol , F > ,) -> syn :: Result < BTreeMap < Symbol , T > > where F : Fn (Symbol , Symbol , & ParseNestedMeta) -> syn :: Result < T > , { let mut result = BTreeMap :: new () ; attr . parse_nested_meta (| meta | get_nested_meta_logic (attr_name , meta , map , & mut result)) ? ; Ok (result) }
};
}
