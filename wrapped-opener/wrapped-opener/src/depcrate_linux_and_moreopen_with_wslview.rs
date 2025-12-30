// Generated macro for open_with_wslview (function)
macro_rules! Depcrate_linux_and_moreopen_with_wslview {
() => {
// Module: crate::linux_and_more
// Provides: {"open_with_wslview"}
// Dependencies: {}
fn open_with_wslview (path : & OsStr) -> io :: Result < Child > { Command :: new ("wslview") . arg (path) . stdin (Stdio :: null ()) . stdout (Stdio :: null ()) . stderr (Stdio :: piped ()) . spawn () }
};
}
