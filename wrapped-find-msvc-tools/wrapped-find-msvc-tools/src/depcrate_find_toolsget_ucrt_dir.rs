// Generated macro for get_ucrt_dir (function)
macro_rules! Depcrate_find_toolsget_ucrt_dir {
() => {
// Module: crate::find_tools
// Provides: {"get_ucrt_dir"}
// Dependencies: {}
# [doc = " To find the Universal CRT we look in a specific registry key for where"] # [doc = " all the Universal CRTs are located and then sort them asciibetically to"] # [doc = " find the newest version. While this sort of sorting isn't ideal,  it is"] # [doc = " what vcvars does so that's good enough for us."] # [doc = ""] # [doc = " Returns a pair of (root, version) for the ucrt dir if found"] pub fn get_ucrt_dir () -> Option < (PathBuf , String) > { impl_ :: get_ucrt_dir () }
};
}
