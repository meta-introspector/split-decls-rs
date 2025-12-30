// Generated macro for PosixLocale (struct)
macro_rules! Depcrate_locale_posixPosixLocale {
() => {
// Module: crate::locale::posix
// Provides: {"PosixLocale"}
// Dependencies: {}
# [derive (Debug)] # [doc = " A parsed and validated POSIX locale identifier."] # [doc = ""] # [doc = " Locales are expected to be in the format `language[_territory][.codeset][@modifier]`;"] # [doc = " only the language section is mandatory, all other sections are optional."] # [doc = " For example:"] # [doc = " - All sections: `en_US.utf8@euro`"] # [doc = " - Only required sections: `en`"] # [doc = ""] # [doc = " See section 8.2 of the POSIX spec for more details:"] # [doc = " <https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap08.html#tag_08_02>"] pub struct PosixLocale < 'src > { language : & 'src str , territory : Option < & 'src str > , codeset : Option < & 'src str > , modifier : Option < & 'src str > , }
};
}
