// Generated macro for find_script (function)
macro_rules! Depcrate_personnames_specifications_derive_localefind_script {
() => {
// Module: crate::personnames::specifications::derive_locale
// Provides: {"find_script"}
// Dependencies: {}
fn find_script < N > (person_name : & N , swe : ScriptWithExtensionsBorrowed , kind : NameFieldKind ,) -> Option < icu_properties :: props :: Script > where N : PersonName , { person_name . available_name_fields () . iter () . filter (| & name_field | name_field . kind == kind) . find_map (| & name_field | { person_name . get (name_field) . chars () . find_map (| c | { let char_script = swe . get_script_val (c) ; match char_script { Script :: Common | Script :: Unknown | Script :: Inherited => None , _ => Some (char_script) , } }) }) }
};
}
