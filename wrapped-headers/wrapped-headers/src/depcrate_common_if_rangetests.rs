// Generated macro for tests (module)
macro_rules! Depcrate_common_if_rangetests {
() => {
// Module: crate::common::if_range
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_is_modified_etag () { let etag = ETag :: from_static ("\"xyzzy\"") ; let if_range = IfRange :: etag (etag . clone ()) ; assert ! (! if_range . is_modified (Some (& etag) , None)) ; let etag = ETag :: from_static ("W/\"xyzzy\"") ; assert ! (if_range . is_modified (Some (& etag) , None)) ; } }
};
}
