// Generated macro for tempdir (function)
macro_rules! Depcrate_core_downloadtempdir {
() => {
// Module: crate::core::download
// Provides: {"tempdir"}
// Dependencies: {}
# [doc = " Create a temporary directory in `out` and return its path."] # [doc = ""] # [doc = " NOTE: this temporary directory is shared between all steps;"] # [doc = " if you need an empty directory, create a new subdirectory inside it."] pub (crate) fn tempdir (out : & Path) -> PathBuf { let tmp = out . join ("tmp") ; t ! (fs :: create_dir_all (& tmp)) ; tmp }
};
}
