// Generated macro for with_current_locale (function)
macro_rules! Depcrate_backends_macoswith_current_locale {
() => {
// Module: crate::backends::macos
// Provides: {"with_current_locale"}
// Dependencies: {}
# [doc = " Helper function to reduce duplication when working with current locale."] # [doc = " Handles the common pattern of getting current locale, using it, and releasing it."] fn with_current_locale < T , F > (f : F) -> Result < Option < T > , HostInfoError > where F : FnOnce (CFLocaleRef) -> Option < T > , { let locale = CFLocaleWrapper :: new () ; Ok (locale . and_then (| loc | f (loc . as_ref ()))) }
};
}
