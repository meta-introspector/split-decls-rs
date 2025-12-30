// Generated macro for reveal (function)
macro_rules! Depcrate_linux_and_morereveal {
() => {
// Module: crate::linux_and_more
// Provides: {"reveal"}
// Dependencies: {}
# [cfg (all (feature = "reveal" , not (target_os = "linux")))] pub (crate) fn reveal (path : & std :: path :: Path) -> Result < () , OpenError > { reveal_fallback (path) }
};
}
