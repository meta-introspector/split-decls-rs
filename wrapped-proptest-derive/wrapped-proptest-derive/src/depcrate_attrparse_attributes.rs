// Generated macro for parse_attributes (function)
macro_rules! Depcrate_attrparse_attributes {
() => {
// Module: crate::attr
// Provides: {"parse_attributes"}
// Dependencies: {}
# [doc = " Parse the attributes specified on an item and parsed by syn"] # [doc = " into our logical model that we work with."] pub fn parse_attributes (ctx : Ctx , attrs : & [Attribute] ,) -> DeriveResult < ParsedAttributes > { let attrs = parse_attributes_base (ctx , attrs) ? ; if attrs . no_bound { error :: no_bound_set_on_non_tyvar (ctx) ; } Ok (attrs) }
};
}
