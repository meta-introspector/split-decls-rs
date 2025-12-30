// Generated macro for list_metadata (function)
macro_rules! Depcratelist_metadata {
() => {
// Module: crate
// Provides: {"list_metadata"}
// Dependencies: {}
fn list_metadata (sess : & Session , metadata_loader : & dyn MetadataLoader) { match sess . io . input { Input :: File (ref ifile) => { let path = & (* ifile) ; let mut v = Vec :: new () ; locator :: list_file_metadata (& sess . target , path , metadata_loader , & mut v , & sess . opts . unstable_opts . ls , sess . cfg_version ,) . unwrap () ; safe_println ! ("{}" , String :: from_utf8 (v) . unwrap ()) ; } Input :: Str { .. } => { # [allow (rustc :: diagnostic_outside_of_impl)] sess . dcx () . fatal ("cannot list metadata for stdin") ; } } }
};
}
