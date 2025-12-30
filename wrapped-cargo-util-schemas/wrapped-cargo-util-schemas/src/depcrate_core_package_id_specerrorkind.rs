// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_core_package_id_specErrorKind {
() => {
// Module: crate::core::package_id_spec
// Provides: {"ErrorKind"}
// Dependencies: {}
# [doc = " Non-public error kind for [`PackageIdSpecError`]."] # [non_exhaustive] # [derive (Debug , thiserror :: Error)] enum ErrorKind { # [error ("unsupported source protocol: {0}")] UnsupportedProtocol (String) , # [error ("`path+{0}` is unsupported; `path+file` and `file` schemes are supported")] UnsupportedPathPlusScheme (String) , # [error ("cannot have a query string in a pkgid: {0}")] UnexpectedQueryString (Url) , # [error ("pkgid urls must have at least one path component: {0}")] MissingUrlPath (Url) , # [error ("package ID specification `{spec}` looks like a file path, maybe try {maybe_url}")] MaybeFilePath { spec : String , maybe_url : String } , # [error (transparent)] NameValidation (# [from] crate :: restricted_names :: NameValidationError) , # [error (transparent)] PartialVersion (# [from] crate :: core :: PartialVersionError) , }
};
}
