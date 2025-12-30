// Generated macro for parse_setlocale_snapshot (function)
macro_rules! Depcrate_backends_shared_posixparse_setlocale_snapshot {
() => {
// Module: crate::backends::shared::posix
// Provides: {"parse_setlocale_snapshot"}
// Dependencies: {}
# [doc = " Attempt to parse `setlocale(LC_ALL, NULL)` into a map."] # [doc = " Returns None if NULL or C/POSIX-like (uninformative), to trigger env fallback."] # [doc = " Note: We only check LC_ALL because if libc is uninitialized, all categories return \"C\"."] # [doc = " If initialized, LC_ALL contains the composite snapshot of all category values."] fn parse_setlocale_snapshot () -> Option < HashMap < LocaleCategory , String > > { let ptr = unsafe { setlocale (LC_ALL , ptr :: null ()) } ; if ptr . is_null () { return None ; } let s = unsafe { CStr :: from_ptr (ptr) } . to_str () . ok () ? ; if s . is_empty () || is_c_like (s) { return None ; } let mut map = HashMap :: new () ; if ! s . contains ('=') { if ! is_c_like (s) { map . insert (LocaleCategory :: All , s . to_string ()) ; } return if map . is_empty () { None } else { Some (map) } ; } for pair in s . split (';') { let mut it = pair . splitn (2 , '=') ; let k = it . next () . unwrap_or_default () . trim () ; let v = it . next () . unwrap_or_default () . trim () ; if v . is_empty () || is_c_like (v) { continue ; } if let Ok (cat) = LocaleCategory :: from_str (k) { map . insert (cat , v . to_string ()) ; } } if map . is_empty () { None } else { Some (map) } }
};
}
