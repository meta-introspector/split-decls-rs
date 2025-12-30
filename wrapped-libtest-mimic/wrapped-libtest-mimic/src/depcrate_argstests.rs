// Generated macro for tests (module)
macro_rules! Depcrate_argstests {
() => {
// Module: crate::args
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn verify_cli () { use clap :: CommandFactory ; Arguments :: command () . debug_assert () ; } }
};
}
