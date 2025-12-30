// Generated macro for check_attributes (function)
macro_rules! Depcrate_internals_attributes_itemcheck_attributes {
() => {
// Module: crate::internals::attributes::item
// Provides: {"check_attributes"}
// Dependencies: {}
pub fn check_attributes (derive_input : & DeriveInput) -> Result < () , Error > { let borsh = get_one_attribute (& derive_input . attrs) ? ; if let Some (attr) = borsh { attr . parse_nested_meta (| meta | { if meta . path != USE_DISCRIMINANT && meta . path != INIT && meta . path != CRATE { return Err (syn :: Error :: new (meta . path . span () , "`crate`, `use_discriminant` or `init` are the only supported attributes for `borsh`" ,)) ; } if meta . path == USE_DISCRIMINANT { let _expr : Expr = meta . value () ? . parse () ? ; if let syn :: Data :: Struct (ref _data) = derive_input . data { return Err (syn :: Error :: new (derive_input . ident . span () , "borsh(use_discriminant=<bool>) does not support structs" ,)) ; } } else if meta . path == INIT || meta . path == CRATE { let _expr : Expr = meta . value () ? . parse () ? ; } Ok (()) }) ? ; } Ok (()) }
};
}
