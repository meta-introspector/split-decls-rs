// Generated macro for impl_685 (impl)
macro_rules! Depcrate_personnames_provided_structimpl_685 {
() => {
// Module: crate::personnames::provided_struct
// Provides: {"impl_685"}
// Dependencies: {}
# [doc = ""] # [doc = " Default person name functions."] impl DefaultPersonName { # [doc = ""] # [doc = " Returns a new person name structure."] pub fn new (person_data : LiteMap < NameField , String > , locale : Option < Locale > , preferred_order : Option < PreferredOrder > ,) -> Result < DefaultPersonName , PersonNamesFormatterError > { let result = DefaultPersonName { person_data , locale , preferred_order , } ; if ! crate :: personnames :: formatter :: validate_person_name (& result . available_name_fields ()) { return Err (PersonNamesFormatterError :: InvalidPersonName) ; } Ok (result) } }
};
}
