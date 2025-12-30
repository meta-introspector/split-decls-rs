// Generated macro for macro_23 (macro)
macro_rules! Depcrate_terminfomacro_23 {
() => {
// Module: crate::terminfo
// Provides: {"macro_23"}
// Dependencies: {}
lazy_static ! { # [doc = " The terminfo database."] static ref TERMINFO : Option < Database > = Database :: from_env () . ok () ; }
};
}
