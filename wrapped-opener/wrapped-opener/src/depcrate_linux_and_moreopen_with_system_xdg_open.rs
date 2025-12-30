// Generated macro for open_with_system_xdg_open (function)
macro_rules! Depcrate_linux_and_moreopen_with_system_xdg_open {
() => {
// Module: crate::linux_and_more
// Provides: {"open_with_system_xdg_open"}
// Dependencies: {}
fn open_with_system_xdg_open (path : & OsStr) -> io :: Result < Child > { Command :: new ("xdg-open") . arg (path) . stdin (Stdio :: null ()) . stdout (Stdio :: null ()) . stderr (Stdio :: null ()) . spawn () }
};
}
