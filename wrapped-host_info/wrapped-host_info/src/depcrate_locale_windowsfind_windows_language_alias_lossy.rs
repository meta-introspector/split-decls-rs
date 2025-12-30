// Generated macro for find_windows_language_alias_lossy (function)
macro_rules! Depcrate_locale_windowsfind_windows_language_alias_lossy {
() => {
// Module: crate::locale::windows
// Provides: {"find_windows_language_alias_lossy"}
// Dependencies: {}
# [doc = " Find a BCP-47 identifier from a list of known Windows aliases."] fn find_windows_language_alias_lossy (lcid : & str) -> Option < LanguageIdentifier > { match lcid { "zh-yue-HK" => Some (langid ! ("yue-HK")) , "x-IV" | "x-IV_mathan" | "x-IV-mathan" => Some (langid ! ("und")) , _ => None , } }
};
}
