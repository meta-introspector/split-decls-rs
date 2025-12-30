// Generated macro for set_title (function)
macro_rules! Depcrate_windows_termset_title {
() => {
// Module: crate::windows_term
// Provides: {"set_title"}
// Dependencies: {}
pub (crate) fn set_title < T : Display > (title : T) { let buffer : Vec < u16 > = OsStr :: new (& format ! ("{title}")) . encode_wide () . chain (once (0)) . collect () ; unsafe { SetConsoleTitleW (buffer . as_ptr ()) ; } }
};
}
