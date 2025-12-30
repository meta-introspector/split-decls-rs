// Generated macro for parse_derive_ex_attrs (function)
macro_rules! Depcrate_item_typeparse_derive_ex_attrs {
() => {
// Module: crate::item_type
// Provides: {"parse_derive_ex_attrs"}
// Dependencies: {}
fn parse_derive_ex_attrs < T : Parse > (attrs : & [Attribute]) -> Result < Vec < T > > { let mut items = Vec :: new () ; for attr in attrs { if attr . path () == & parse_quote ! (derive_ex) { items . push (attr . parse_args () ?) ; } } Ok (items) }
};
}
