// Generated macro for OidArray (struct)
macro_rules! Depcrate_oid_arrayOidArray {
() => {
// Module: crate::oid_array
// Provides: {"OidArray"}
// Dependencies: {}
# [doc = " An oid array structure used by libgit2"] # [doc = ""] # [doc = " Some APIs return arrays of OIDs which originate from libgit2. This"] # [doc = " wrapper type behaves a little like `Vec<&Oid>` but does so without copying"] # [doc = " the underlying Oids until necessary."] pub struct OidArray { raw : raw :: git_oidarray , }
};
}
