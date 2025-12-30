// Generated macro for display (function)
macro_rules! Depcrate_exportdisplay {
() => {
// Module: crate::export
// Provides: {"display"}
// Dependencies: {}
# [doc = " Implementation detail"] pub fn display (val : & dyn core :: fmt :: Display) { core :: write ! (FmtWrite , "{val}") . ok () ; write (& [0xff]) ; }
};
}
