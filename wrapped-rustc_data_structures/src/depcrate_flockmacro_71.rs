// Generated macro for macro_71 (macro)
macro_rules! Depcrate_flockmacro_71 {
() => {
// Module: crate::flock
// Provides: {"macro_71"}
// Dependencies: {}
cfg_select ! { target_os = "linux" => { mod linux ; use linux as imp ; } target_os = "redox" => { mod linux ; use linux as imp ; } unix => { mod unix ; use unix as imp ; } windows => { mod windows ; use self :: windows as imp ; } _ => { mod unsupported ; use unsupported as imp ; } }
};
}
