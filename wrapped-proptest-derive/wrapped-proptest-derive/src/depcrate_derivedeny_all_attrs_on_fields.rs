// Generated macro for deny_all_attrs_on_fields (function)
macro_rules! Depcrate_derivedeny_all_attrs_on_fields {
() => {
// Module: crate::derive
// Provides: {"deny_all_attrs_on_fields"}
// Dependencies: {}
# [doc = " Ensures that there are no proptest attributes on any of the fields."] fn deny_all_attrs_on_fields (ctx : Ctx , fields : Vec < Field >) -> DeriveResult < () > { fields . into_iter () . try_for_each (| field | { let f_attr = attr :: parse_attributes (ctx , & field . attrs) ? ; error :: if_anything_specified (ctx , & f_attr , error :: ENUM_VARIANT_FIELD) ; Ok (()) }) }
};
}
