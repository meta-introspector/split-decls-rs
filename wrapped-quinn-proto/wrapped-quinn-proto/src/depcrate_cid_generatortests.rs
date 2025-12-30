// Generated macro for tests (module)
macro_rules! Depcrate_cid_generatortests {
() => {
// Module: crate::cid_generator
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn validate_keyed_cid () { let mut generator = HashedConnectionIdGenerator :: new () ; let cid = generator . generate_cid () ; generator . validate (cid) . unwrap () ; } }
};
}
