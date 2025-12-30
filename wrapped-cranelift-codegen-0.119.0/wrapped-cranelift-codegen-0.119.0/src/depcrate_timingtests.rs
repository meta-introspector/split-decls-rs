// Generated macro for tests (module)
macro_rules! Depcrate_timingtests {
() => {
// Module: crate::timing
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use alloc :: string :: ToString ; # [test] fn display () { assert_eq ! (Pass :: None . to_string () , "<no pass>") ; assert_eq ! (Pass :: regalloc . to_string () , "Register allocation") ; } }
};
}
