// Generated macro for tests (module)
macro_rules! Depcrate_randtests {
() => {
// Module: crate::rand
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] fn test_rand_bytes () { let mut buf = [0 ; 32] ; super :: rand_bytes (& mut buf) . unwrap () ; } # [test] # [cfg (ossl111)] fn test_rand_priv_bytes () { let mut buf = [0 ; 32] ; super :: rand_priv_bytes (& mut buf) . unwrap () ; } }
};
}
