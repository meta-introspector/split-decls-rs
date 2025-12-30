// Generated macro for resolve_env_for_category (function)
macro_rules! Depcrate_backends_shared_posixresolve_env_for_category {
() => {
// Module: crate::backends::shared::posix
// Provides: {"resolve_env_for_category"}
// Dependencies: {}
# [doc = " POSIX precedence: LC_ALL > LC_<CAT> > LANG."] # [doc = " Returns Some(non-C/POSIX) or None if unset/C-like."] fn resolve_env_for_category (cat : LocaleCategory) -> Option < String > { if let Some (v) = non_c_like_env ("LC_ALL") { return Some (v) ; } if cat != LocaleCategory :: All { if let Some (v) = non_c_like_env (cat . to_env_var_name ()) { return Some (v) ; } } non_c_like_env ("LANG") }
};
}
