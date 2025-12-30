// Generated macro for impl_394 (impl)
macro_rules! Depcrate_displaynames_displaynamesimpl_394 {
() => {
// Module: crate::displaynames::displaynames
// Provides: {"impl_394"}
// Dependencies: {}
impl ScriptDisplayNames { icu_provider :: gen_buffer_data_constructors ! ((prefs : DisplayNamesPreferences , options : DisplayNamesOptions) -> error : DataError , # [doc = " Creates a new [`ScriptDisplayNames`] from locale data and an options bag using compiled data."] functions : [try_new , try_new_with_buffer_provider , try_new_unstable , Self]) ; # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: try_new)] pub fn try_new_unstable < D : DataProvider < ScriptDisplayNamesV1 > + ? Sized > (provider : & D , prefs : DisplayNamesPreferences , options : DisplayNamesOptions ,) -> Result < Self , DataError > { let locale = ScriptDisplayNamesV1 :: make_locale (prefs . locale_preferences) ; let script_data = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_locale (& locale) , .. Default :: default () }) ? . payload ; Ok (Self { options , script_data , }) } # [doc = " Returns the display name of a script."] pub fn of (& self , script : Script) -> Option < & str > { let data = self . script_data . get () ; match self . options . style { Some (Style :: Short) => data . short_names . get (& script . to_tinystr () . to_unvalidated ()) , _ => None , } . or_else (| | data . names . get (& script . to_tinystr () . to_unvalidated ())) } }
};
}
