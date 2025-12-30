// Generated macro for has_no_bound (function)
macro_rules! Depcrate_attrhas_no_bound {
() => {
// Module: crate::attr
// Provides: {"has_no_bound"}
// Dependencies: {}
# [doc = " Parses the attributes specified on an item and parsed by syn"] # [doc = " and returns true if we've been ordered to not set an `Arbitrary`"] # [doc = " bound on the given type variable the attributes are from,"] # [doc = " no matter what."] pub fn has_no_bound (ctx : Ctx , attrs : & [Attribute]) -> DeriveResult < bool > { let attrs = parse_attributes_base (ctx , attrs) ? ; error :: if_anything_specified (ctx , & attrs , error :: TY_VAR) ; Ok (attrs . no_bound) }
};
}
