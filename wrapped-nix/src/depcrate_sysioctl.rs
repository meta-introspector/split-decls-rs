// Generated macro for ioctl (module)
macro_rules! Depcrate_sysioctl {
() => {
// Module: crate::sys
// Provides: {"ioctl"}
// Dependencies: {}
# [cfg (any (bsd , linux_android , solarish , target_os = "fuchsia" , target_os = "redox" ,))] # [cfg (feature = "ioctl")] # [cfg_attr (docsrs , doc (cfg (feature = "ioctl")))] # [macro_use] pub mod ioctl ;
};
}
