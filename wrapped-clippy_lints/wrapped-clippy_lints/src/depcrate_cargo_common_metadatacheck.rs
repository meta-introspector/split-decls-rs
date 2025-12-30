// Generated macro for check (function)
macro_rules! Depcrate_cargo_common_metadatacheck {
() => {
// Module: crate::cargo::common_metadata
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , metadata : & Metadata , ignore_publish : bool) { for package in & metadata . packages { if package . publish . as_ref () . filter (| publish | publish . is_empty ()) . is_none () || ignore_publish { if is_empty_str (package . description . as_ref ()) { missing_warning (cx , package , "package.description") ; } if is_empty_str (package . license . as_ref ()) && is_empty_str (package . license_file . as_ref ()) { missing_warning (cx , package , "either package.license or package.license_file") ; } if is_empty_str (package . repository . as_ref ()) { missing_warning (cx , package , "package.repository") ; } if is_empty_str (package . readme . as_ref ()) { missing_warning (cx , package , "package.readme") ; } if is_empty_vec (package . keywords . as_ref ()) { missing_warning (cx , package , "package.keywords") ; } if is_empty_vec (package . categories . as_ref ()) { missing_warning (cx , package , "package.categories") ; } } } }
};
}
