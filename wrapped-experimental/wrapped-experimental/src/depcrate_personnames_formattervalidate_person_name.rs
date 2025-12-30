// Generated macro for validate_person_name (function)
macro_rules! Depcrate_personnames_formattervalidate_person_name {
() => {
// Module: crate::personnames::formatter
// Provides: {"validate_person_name"}
// Dependencies: {}
# [doc = " Validate that the provided fields are valid."] # [doc = " If the person name is not valid, it will not be formatted."] pub (crate) fn validate_person_name (available_name_fields : & [& NameField]) -> bool { available_name_fields . iter () . any (| field | field . kind == NameFieldKind :: Given || field . kind == NameFieldKind :: Surname) }
};
}
