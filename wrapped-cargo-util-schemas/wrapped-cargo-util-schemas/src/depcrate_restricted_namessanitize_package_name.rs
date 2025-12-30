// Generated macro for sanitize_package_name (function)
macro_rules! Depcrate_restricted_namessanitize_package_name {
() => {
// Module: crate::restricted_names
// Provides: {"sanitize_package_name"}
// Dependencies: {}
# [doc = " Ensure a package name is [valid][validate_package_name]"] pub (crate) fn sanitize_package_name (name : & str , placeholder : char) -> String { let mut slug = String :: new () ; for part in name . split ("::") { if ! slug . is_empty () { slug . push_str ("::") ; } slug . push_str (& sanitize_name (part , placeholder)) ; } slug }
};
}
