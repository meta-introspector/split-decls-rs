// Generated macro for validate_package_name (function)
macro_rules! Depcrate_restricted_namesvalidate_package_name {
() => {
// Module: crate::restricted_names
// Provides: {"validate_package_name"}
// Dependencies: {}
pub (crate) fn validate_package_name (name : & str) -> Result < () > { for part in name . split ("::") { validate_name (part , "package name") ? ; } Ok (()) }
};
}
