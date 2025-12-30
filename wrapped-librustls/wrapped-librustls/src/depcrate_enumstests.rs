// Generated macro for tests (module)
macro_rules! Depcrate_enumstests {
() => {
// Module: crate::enums
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use rustls :: { ALL_VERSIONS , DEFAULT_VERSIONS } ; use super :: * ; # [test] fn all_versions_arrays () { assert_eq ! (RUSTLS_ALL_VERSIONS_LEN , ALL_VERSIONS . len ()) ; for (original , ffi) in ALL_VERSIONS . iter () . zip (RUSTLS_ALL_VERSIONS . iter ()) { assert_eq ! (u16 :: from (original . version) , * ffi) ; } } # [test] fn default_versions_arrays () { assert_eq ! (RUSTLS_DEFAULT_VERSIONS_LEN , DEFAULT_VERSIONS . len ()) ; for (original , ffi) in DEFAULT_VERSIONS . iter () . zip (RUSTLS_DEFAULT_VERSIONS . iter ()) { assert_eq ! (u16 :: from (original . version) , * ffi) ; } } }
};
}
