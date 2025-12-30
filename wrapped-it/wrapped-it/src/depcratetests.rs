// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn clap () { use clap :: CommandFactory ; Args :: command () . debug_assert () ; } }
};
}
