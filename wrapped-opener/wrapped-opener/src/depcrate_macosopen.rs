// Generated macro for open (function)
macro_rules! Depcrate_macosopen {
() => {
// Module: crate::macos
// Provides: {"open"}
// Dependencies: {}
pub (crate) fn open (path : & OsStr) -> Result < () , OpenError > { let mut open = Command :: new ("open") . arg (path) . stdin (Stdio :: null ()) . stdout (Stdio :: null ()) . stderr (Stdio :: piped ()) . spawn () . map_err (OpenError :: Io) ? ; crate :: wait_child (& mut open , "open") }
};
}
