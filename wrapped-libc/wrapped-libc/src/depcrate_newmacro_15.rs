// Generated macro for macro_15 (macro)
macro_rules! Depcrate_newmacro_15 {
() => {
// Module: crate::new
// Provides: {"macro_15"}
// Dependencies: {}
cfg_if ! { if # [cfg (target_os = "linux")] { mod linux_uapi ; pub use linux_uapi ::*; } else if # [cfg (target_os = "android")] { mod bionic ; pub use bionic ::*; } }
};
}
