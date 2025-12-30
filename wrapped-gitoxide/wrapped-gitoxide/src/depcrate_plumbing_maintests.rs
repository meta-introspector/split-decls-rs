// Generated macro for tests (module)
macro_rules! Depcrate_plumbing_maintests {
() => {
// Module: crate::plumbing::main
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn clap () { use clap :: CommandFactory ; Args :: command () . debug_assert () ; } }
};
}
