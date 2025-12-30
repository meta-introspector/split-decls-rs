// Generated macro for impl_47 (impl)
macro_rules! Depcrate_internals_attributes_fieldimpl_47 {
() => {
// Module: crate::internals::attributes::field
// Provides: {"impl_47"}
// Dependencies: {}
impl Attributes { fn check (& self , attr : & Attribute) -> Result < () , syn :: Error > { if self . skip && (self . serialize_with . is_some () || self . deserialize_with . is_some ()) { return Err (syn :: Error :: new_spanned (attr , format ! ("`{}` cannot be used at the same time as `{}` or `{}`" , SKIP . 0 , SERIALIZE_WITH . 0 , DESERIALIZE_WITH . 0) ,)) ; } # [cfg (feature = "schema")] self . check_schema (attr) ? ; Ok (()) } pub (crate) fn parse (attrs : & [Attribute]) -> Result < Self , syn :: Error > { let borsh = get_one_attribute (attrs) ? ; let result : Self = if let Some (attr) = borsh { let result : Self = attr_get_by_symbol_keys (BORSH , attr , & BORSH_FIELD_PARSE_MAP) ? . into () ; result . check (attr) ? ; result } else { BTreeMap :: new () . into () } ; Ok (result) } pub (crate) fn needs_bounds_derive (& self , ty : BoundType) -> bool { let predicates = self . get_bounds (ty) ; predicates . is_none () } fn get_bounds (& self , ty : BoundType) -> Option < Vec < WherePredicate > > { let bounds = self . bounds . as_ref () ; bounds . and_then (| bounds | match ty { BoundType :: Serialize => bounds . serialize . clone () , BoundType :: Deserialize => bounds . deserialize . clone () , }) } pub (crate) fn collect_bounds (& self , ty : BoundType) -> Vec < WherePredicate > { let predicates = self . get_bounds (ty) ; predicates . unwrap_or_default () } }
};
}
