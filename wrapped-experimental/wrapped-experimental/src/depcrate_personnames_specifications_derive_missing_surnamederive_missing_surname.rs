// Generated macro for derive_missing_surname (function)
macro_rules! Depcrate_personnames_specifications_derive_missing_surnamederive_missing_surname {
() => {
// Module: crate::personnames::specifications::derive_missing_surname
// Provides: {"derive_missing_surname"}
// Dependencies: {}
# [doc = " Returns a remapped field for missing surname."] # [doc = " https://www.unicode.org/reports/tr35/tr35-personNames.html#handle-missing-surname"] pub fn derive_missing_surname (available_name_field : & [& NameField] , requested_name_field : & NameField , requires_given_name : bool ,) -> Option < NameField > { match requested_name_field . kind { NameFieldKind :: Surname | NameFieldKind :: Given => { let has_surname = available_name_field . iter () . any (| & field | field . kind == NameFieldKind :: Surname) ; if ! has_surname && ! requires_given_name { if requested_name_field . kind == NameFieldKind :: Surname { return Some (NameField { kind : NameFieldKind :: Given , modifier : requested_name_field . modifier , }) ; } return None ; } Some (* requested_name_field) } _ => Some (* requested_name_field) , } }
};
}
