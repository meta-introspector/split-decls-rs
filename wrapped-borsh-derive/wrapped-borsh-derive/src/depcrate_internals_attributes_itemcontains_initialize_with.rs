// Generated macro for contains_initialize_with (function)
macro_rules! Depcrate_internals_attributes_itemcontains_initialize_with {
() => {
// Module: crate::internals::attributes::item
// Provides: {"contains_initialize_with"}
// Dependencies: {}
pub (crate) fn contains_initialize_with (attrs : & [Attribute]) -> Result < Option < Path > , Error > { let mut res = None ; let attr = attrs . iter () . find (| attr | attr . path () == BORSH) ; if let Some (attr) = attr { attr . parse_nested_meta (| meta | { if meta . path == INIT { let value_expr : Path = meta . value () ? . parse () ? ; res = Some (value_expr) ; } else if meta . path == USE_DISCRIMINANT || meta . path == CRATE { let _value_expr : Expr = meta . value () ? . parse () ? ; } Ok (()) }) ? ; } Ok (res) }
};
}
