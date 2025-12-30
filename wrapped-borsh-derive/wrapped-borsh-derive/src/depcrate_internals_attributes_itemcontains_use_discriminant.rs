// Generated macro for contains_use_discriminant (function)
macro_rules! Depcrate_internals_attributes_itemcontains_use_discriminant {
() => {
// Module: crate::internals::attributes::item
// Provides: {"contains_use_discriminant"}
// Dependencies: {}
pub (crate) fn contains_use_discriminant (input : & ItemEnum) -> Result < bool , syn :: Error > { if input . variants . len () > 256 { return Err (syn :: Error :: new (input . span () , "up to 256 enum variants are supported" ,)) ; } let attrs = & input . attrs ; let mut use_discriminant = None ; let attr = attrs . iter () . find (| attr | attr . path () == BORSH) ; if let Some (attr) = attr { attr . parse_nested_meta (| meta | { if meta . path == USE_DISCRIMINANT { let value_expr : Expr = meta . value () ? . parse () ? ; let value = value_expr . to_token_stream () . to_string () ; match value . as_str () { "true" => { use_discriminant = Some (true) ; } "false" => use_discriminant = Some (false) , _ => { return Err (syn :: Error :: new (value_expr . span () , "`use_discriminant` accepts only `true` or `false`" ,)) ; } } ; } else if meta . path == INIT || meta . path == CRATE { let _value_expr : Expr = meta . value () ? . parse () ? ; } Ok (()) }) ? ; } let has_explicit_discriminants = input . variants . iter () . any (| variant | variant . discriminant . is_some ()) ; if has_explicit_discriminants && use_discriminant . is_none () { return Err (syn :: Error :: new (input . ident . span () , "You have to specify `#[borsh(use_discriminant=true)]` or `#[borsh(use_discriminant=false)]` for all enums with explicit discriminant" ,)) ; } Ok (use_discriminant . unwrap_or (false)) }
};
}
