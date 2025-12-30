// Generated macro for test (module)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn verify_app () { # [derive (Debug , clap :: Parser)] struct Cli { # [clap (flatten)] color : Color , } use clap :: CommandFactory ; Cli :: command () . debug_assert () ; } }
};
}
