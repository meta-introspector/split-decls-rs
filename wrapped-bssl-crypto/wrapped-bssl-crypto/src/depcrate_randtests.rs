// Generated macro for tests (module)
macro_rules! Depcrate_randtests {
() => {
// Module: crate::rand
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn fill () { let mut buf = [0 ; 32] ; rand_bytes (& mut buf) ; } # [test] fn fill_empty () { let mut buf = [] ; rand_bytes (& mut buf) ; } # [test] fn array () { let _rand : [u8 ; 32] = rand_array () ; } # [test] fn empty_array () { let _rand : [u8 ; 0] = rand_array () ; } }
};
}
