// Generated macro for test (module)
macro_rules! Depcrate_portabletest {
() => {
// Module: crate::portable
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] pub mod test { use super :: * ; # [test] fn test_compress () { crate :: test :: test_compress_fn (compress_in_place , compress_xof) ; } # [test] fn test_hash_many () { crate :: test :: test_hash_many_fn (hash_many , hash_many) ; } }
};
}
