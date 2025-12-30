// Generated macro for ReadDir (struct)
macro_rules! Depcrate_core_read_dirReadDir {
() => {
// Module: crate::core::read_dir
// Provides: {"ReadDir"}
// Dependencies: {}
# [doc = " Results of successfully reading a directory."] # [derive (Debug)] pub struct ReadDir < C : ClientState > { pub (crate) read_dir_state : C :: ReadDirState , pub (crate) results_list : Vec < Result < DirEntry < C > > > , }
};
}
