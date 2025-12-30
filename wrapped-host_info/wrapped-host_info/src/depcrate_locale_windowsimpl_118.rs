// Generated macro for impl_118 (impl)
macro_rules! Depcrate_locale_windowsimpl_118 {
() => {
// Module: crate::locale::windows
// Provides: {"impl_118"}
// Dependencies: {}
impl < 'src > TryFrom < WindowsLocale < 'src > > for Locale { type Error = ParseError ; fn try_from (input : WindowsLocale < 'src >) -> Result < Self , Self :: Error > { let (lcid , collation_value) = strip_windows_collation_suffix_lossy (input . src) ; let keywords = match collation_value { Some (collation_value) => Keywords :: new_single (key ! ("co") , collation_value) , None => Keywords :: new () , } ; let language = match find_windows_language_alias_lossy (lcid) { Some (locale) => locale , None => LanguageIdentifier :: try_from_str (lcid) ? , } ; Ok (Locale { id : language , extensions : Extensions :: from_unicode (Unicode { keywords , .. Unicode :: new () }) , }) } }
};
}
