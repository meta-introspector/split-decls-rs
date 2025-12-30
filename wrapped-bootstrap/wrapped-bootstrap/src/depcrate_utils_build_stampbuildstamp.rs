// Generated macro for BuildStamp (struct)
macro_rules! Depcrate_utils_build_stampBuildStamp {
() => {
// Module: crate::utils::build_stamp
// Provides: {"BuildStamp"}
// Dependencies: {}
# [doc = " Manages a stamp file to track build state. The file is created in the given"] # [doc = " directory and can have custom content and name."] # [derive (Clone)] pub struct BuildStamp { path : PathBuf , stamp : String , }
};
}
