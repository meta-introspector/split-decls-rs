// Generated macro for IS_LANG_UTF8 (static)
macro_rules! Depcrate_unix_termIS_LANG_UTF8 {
() => {
// Module: crate::unix_term
// Provides: {"IS_LANG_UTF8"}
// Dependencies: {}
# [cfg (not (target_os = "macos"))] static IS_LANG_UTF8 : Lazy < bool > = Lazy :: new (| | match std :: env :: var ("LANG") { Ok (lang) => lang . to_uppercase () . ends_with ("UTF-8") , _ => false , }) ;
};
}
