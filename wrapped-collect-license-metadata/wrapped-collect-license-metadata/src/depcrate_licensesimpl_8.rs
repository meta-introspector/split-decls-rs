// Generated macro for impl_8 (impl)
macro_rules! Depcrate_licensesimpl_8 {
() => {
// Module: crate::licenses
// Provides: {"impl_8"}
// Dependencies: {}
impl License { fn simplify (& mut self) { self . remove_copyright_prefixes () ; self . remove_trailing_dots () ; self . copyright . sort () ; self . copyright . dedup () ; } fn remove_copyright_prefixes (& mut self) { for copyright in & mut self . copyright { let mut stripped = copyright . trim () ; let mut previous_stripped ; loop { previous_stripped = stripped ; for pattern in COPYRIGHT_PREFIXES { stripped = stripped . trim_start_matches (pattern) . trim_start () ; } if stripped == previous_stripped { break ; } } * copyright = stripped . into () ; } } fn remove_trailing_dots (& mut self) { for copyright in & mut self . copyright { if copyright . ends_with ('.') { * copyright = copyright . trim_end_matches ('.') . to_string () ; } } } }
};
}
