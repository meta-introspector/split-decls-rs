// Generated macro for show_version (function)
macro_rules! Depcrate_phasesshow_version {
() => {
// Module: crate::phases
// Provides: {"show_version"}
// Dependencies: {}
fn show_version () { print ! ("miri {}" , env ! ("CARGO_PKG_VERSION")) ; let version = format ! ("{} {}" , env ! ("GIT_HASH") , env ! ("COMMIT_DATE")) ; if version . len () > 1 { print ! (" ({version})") ; } println ! () ; }
};
}
