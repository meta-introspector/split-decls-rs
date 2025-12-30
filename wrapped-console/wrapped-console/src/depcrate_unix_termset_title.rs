// Generated macro for set_title (function)
macro_rules! Depcrate_unix_termset_title {
() => {
// Module: crate::unix_term
// Provides: {"set_title"}
// Dependencies: {}
pub (crate) fn set_title < T : Display > (title : T) { print ! ("\x1b]0;{title}\x07") ; }
};
}
