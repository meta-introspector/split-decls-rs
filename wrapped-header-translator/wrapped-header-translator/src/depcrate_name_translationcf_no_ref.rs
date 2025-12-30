// Generated macro for cf_no_ref (function)
macro_rules! Depcrate_name_translationcf_no_ref {
() => {
// Module: crate::name_translation
// Provides: {"cf_no_ref"}
// Dependencies: {}
pub (crate) fn cf_no_ref (type_name : & str) -> & str { type_name . strip_suffix ("Ref") . unwrap_or (type_name) }
};
}
