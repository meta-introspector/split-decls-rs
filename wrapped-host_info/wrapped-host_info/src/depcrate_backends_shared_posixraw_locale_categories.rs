// Generated macro for raw_locale_categories (function)
macro_rules! Depcrate_backends_shared_posixraw_locale_categories {
() => {
// Module: crate::backends::shared::posix
// Provides: {"raw_locale_categories"}
// Dependencies: {}
# [doc = " Retrieves locales for LC_ALL and any explicitly-set categories in this thread."] # [doc = " If libc is uninitialized (NULL/C/POSIX), falls back to env precedence."] # [doc = " If nothing resolves, returns `{ LC_ALL: \"en-US-posix\" }`."] pub (crate) fn raw_locale_categories () -> Result < HashMap < LocaleCategory , String > , HostInfoError > { if let Some (map) = parse_setlocale_snapshot () { return Ok (map) ; } const CATS : & [LocaleCategory] = & [LocaleCategory :: Character , LocaleCategory :: Number , LocaleCategory :: Time , LocaleCategory :: Collate , LocaleCategory :: Monetary , LocaleCategory :: Messages , LocaleCategory :: Paper , LocaleCategory :: Name , LocaleCategory :: Address , LocaleCategory :: Telephone , LocaleCategory :: Measurement , LocaleCategory :: Identification , LocaleCategory :: All ,] ; let mut out = HashMap :: new () ; for & cat in CATS { if let Some (v) = resolve_env_for_category (cat) { out . insert (cat , v) ; } } if out . is_empty () { out . insert (LocaleCategory :: All , "en-US-posix" . to_string ()) ; } Ok (out) }
};
}
