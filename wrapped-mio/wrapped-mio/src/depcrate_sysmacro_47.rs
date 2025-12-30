// Generated macro for macro_47 (macro)
macro_rules! Depcrate_sysmacro_47 {
() => {
// Module: crate::sys
// Provides: {"macro_47"}
// Dependencies: {}
cfg_not_os_poll ! { mod shell ; pub (crate) use self :: shell ::*; # [cfg (unix)] cfg_any_os_ext ! { mod unix ; # [cfg (feature = "os-ext")] pub use self :: unix :: SourceFd ; } }
};
}
