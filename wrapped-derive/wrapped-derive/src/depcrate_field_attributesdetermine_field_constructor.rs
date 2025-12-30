// Generated macro for determine_field_constructor (function)
macro_rules! Depcrate_field_attributesdetermine_field_constructor {
() => {
// Module: crate::field_attributes
// Provides: {"determine_field_constructor"}
// Dependencies: {}
pub fn determine_field_constructor (field : & Field) -> Result < FieldConstructor > { let opt_attr = fetch_attr_from_field (field) ? ; let ctor = match opt_attr { Some (attr) => parse_attribute (attr) ? , None => FieldConstructor :: Arbitrary , } ; Ok (ctor) }
};
}
