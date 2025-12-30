// Generated macro for run (function)
macro_rules! Depcrate_cleanrun {
() => {
// Module: crate::clean
// Provides: {"run"}
// Dependencies: {}
pub fn run () -> Result < () , String > { match CleanArg :: new () ? { CleanArg :: All => clean_all () ? , CleanArg :: UiTests => clean_ui_tests () ? , CleanArg :: Help => usage () , } Ok (()) }
};
}
