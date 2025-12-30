// Generated macro for debug (function)
macro_rules! Depcrate_exportdebug {
() => {
// Module: crate::export
// Provides: {"debug"}
// Dependencies: {}
# [doc = " Implementation detail"] pub fn debug (val : & dyn core :: fmt :: Debug) { core :: write ! (FmtWrite , "{val:?}") . ok () ; write (& [0xff]) ; }
};
}
