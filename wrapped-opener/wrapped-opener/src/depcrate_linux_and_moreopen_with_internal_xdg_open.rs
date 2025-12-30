// Generated macro for open_with_internal_xdg_open (function)
macro_rules! Depcrate_linux_and_moreopen_with_internal_xdg_open {
() => {
// Module: crate::linux_and_more
// Provides: {"open_with_internal_xdg_open"}
// Dependencies: {}
fn open_with_internal_xdg_open (path : & OsStr) -> Result < Child , OpenError > { let mut sh = Command :: new ("sh") . arg ("-s") . arg (path) . stdin (Stdio :: piped ()) . stdout (Stdio :: null ()) . stderr (Stdio :: null ()) . spawn () . map_err (| err | OpenError :: Spawn { cmds : "sh" . into () , source : err , }) ? ; sh . stdin . as_mut () . unwrap () . write_all (XDG_OPEN_SCRIPT) . map_err (OpenError :: Io) ? ; Ok (sh) }
};
}
