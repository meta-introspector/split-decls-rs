// Generated macro for get_one_attribute (function)
macro_rules! Depcrate_internals_attributesget_one_attribute {
() => {
// Module: crate::internals::attributes
// Provides: {"get_one_attribute"}
// Dependencies: {}
fn get_one_attribute (attrs : & [Attribute]) -> syn :: Result < Option < & Attribute > > { let count = attrs . iter () . filter (| attr | attr . path () == BORSH) . count () ; let borsh = attrs . iter () . find (| attr | attr . path () == BORSH) ; if count > 1 { return Err (syn :: Error :: new_spanned (borsh . unwrap () , format ! ("multiple `{}` attributes not allowed" , BORSH . 0) ,)) ; } Ok (borsh) }
};
}
