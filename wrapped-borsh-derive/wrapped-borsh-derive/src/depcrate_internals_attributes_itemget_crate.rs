// Generated macro for get_crate (function)
macro_rules! Depcrate_internals_attributes_itemget_crate {
() => {
// Module: crate::internals::attributes::item
// Provides: {"get_crate"}
// Dependencies: {}
pub (crate) fn get_crate (attrs : & [Attribute]) -> Result < Option < Path > , Error > { let mut res = None ; let attr = attrs . iter () . find (| attr | attr . path () == BORSH) ; if let Some (attr) = attr { attr . parse_nested_meta (| meta | { if meta . path == CRATE { let value_expr : Path = parsing :: parse_lit_into (BORSH , CRATE , & meta) ? ; res = Some (value_expr) ; } else if meta . path == USE_DISCRIMINANT || meta . path == INIT { let _value_expr : Expr = meta . value () ? . parse () ? ; } Ok (()) }) ? ; } Ok (res) }
};
}
