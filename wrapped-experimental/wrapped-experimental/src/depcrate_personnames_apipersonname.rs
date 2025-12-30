// Generated macro for PersonName (trait)
macro_rules! Depcrate_personnames_apiPersonName {
() => {
// Module: crate::personnames::api
// Provides: {"PersonName"}
// Dependencies: {}
# [doc = " Trait for providing person name data."] pub trait PersonName { # [doc = " Returns the name locale of person name."] fn name_locale (& self) -> Option < & Locale > ; # [doc = " Returns the preferred order of person name."] fn preferred_order (& self) -> Option < & PreferredOrder > ; # [doc = " Returns the value of the given field name, it *must* match the name field requested."] # [doc = " The string should be in NFC."] fn get (& self , field : & NameField) -> & str ; # [doc = " Returns all available name field."] fn available_name_fields (& self) -> Vec < & NameField > ; # [doc = " Returns true if the provided field name is available."] fn has_name_field_kind (& self , lookup_name_field : & NameFieldKind) -> bool ; # [doc = " Returns true if person have the name field matching the type and modifier."] fn has_name_field (& self , lookup_name_field : & NameField) -> bool ; }
};
}
