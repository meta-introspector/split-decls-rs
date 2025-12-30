// Generated macro for reveal_fallback (function)
macro_rules! Depcrate_linux_and_morereveal_fallback {
() => {
// Module: crate::linux_and_more
// Provides: {"reveal_fallback"}
// Dependencies: {}
# [cfg (feature = "reveal")] fn reveal_fallback (path : & std :: path :: Path) -> Result < () , OpenError > { let path = path . canonicalize () . map_err (OpenError :: Io) ? ; let parent = path . parent () . unwrap_or (std :: path :: Path :: new ("/")) ; open (parent . as_os_str ()) }
};
}
