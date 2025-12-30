// Generated macro for impl_684 (impl)
macro_rules! Depcrate_personnames_provided_structimpl_684 {
() => {
// Module: crate::personnames::provided_struct
// Provides: {"impl_684"}
// Dependencies: {}
impl PersonName for DefaultPersonName { fn name_locale (& self) -> Option < & Locale > { self . locale . as_ref () } fn preferred_order (& self) -> Option < & PreferredOrder > { self . preferred_order . as_ref () } fn get (& self , field : & NameField) -> & str { self . person_data . get (field) . map (String :: as_ref) . unwrap_or ("") } fn available_name_fields (& self) -> Vec < & NameField > { self . person_data . keys () . collect () } fn has_name_field_kind (& self , lookup_name_field : & NameFieldKind) -> bool { self . available_name_fields () . into_iter () . any (| field | & field . kind == lookup_name_field) } fn has_name_field (& self , lookup_name_field : & NameField) -> bool { self . available_name_fields () . into_iter () . any (| field | field == lookup_name_field) } }
};
}
