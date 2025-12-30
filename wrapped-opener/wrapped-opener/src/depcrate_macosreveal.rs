// Generated macro for reveal (function)
macro_rules! Depcrate_macosreveal {
() => {
// Module: crate::macos
// Provides: {"reveal"}
// Dependencies: {}
# [cfg (feature = "reveal")] pub (crate) fn reveal (path : & std :: path :: Path) -> Result < () , OpenError > { let mut open = Command :: new ("open") . arg ("-R") . arg ("--") . arg (path) . stdin (Stdio :: null ()) . stdout (Stdio :: null ()) . stderr (Stdio :: piped ()) . spawn () . map_err (OpenError :: Io) ? ; crate :: wait_child (& mut open , "open") }
};
}
