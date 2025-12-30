// Generated macro for Dir (struct)
macro_rules! Depcrate_fs_dirDir {
() => {
// Module: crate::fs::dir
// Provides: {"Dir"}
// Dependencies: {}
# [doc = " A **Dir** provides a cached list of the file paths in a directory that’s"] # [doc = " being listed."] # [doc = ""] # [doc = " This object gets passed to the Files themselves, in order for them to"] # [doc = " check the existence of surrounding files, then highlight themselves"] # [doc = " accordingly. (See `File#get_source_files`)"] pub struct Dir { # [doc = " A vector of the files that have been read from this directory."] contents : Vec < DirEntry > , # [doc = " The path that was read."] pub path : PathBuf , }
};
}
