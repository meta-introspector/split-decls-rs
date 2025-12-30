// Generated macro for DirectorySourceOptions (struct)
macro_rules! Depcrate_registryDirectorySourceOptions {
() => {
// Module: crate::registry
// Provides: {"DirectorySourceOptions"}
// Dependencies: {}
# [doc = " Options for importing template files from a directory."] # [non_exhaustive] # [derive (Builder)] # [builder (default)] # [cfg (feature = "dir_source")] pub struct DirectorySourceOptions { # [doc = " The name extension for template files"] # [builder (setter (into))] pub tpl_extension : String , # [doc = " Whether to include hidden files (file name that starts with `.`)"] pub hidden : bool , # [doc = " Whether to include temporary files (file name that starts with `#`)"] pub temporary : bool , }
};
}
