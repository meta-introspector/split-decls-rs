// Generated macro for PackageFile (enum)
macro_rules! Depcrate_messagesPackageFile {
() => {
// Module: crate::messages
// Provides: {"PackageFile"}
// Dependencies: {}
# [doc = " Where the file is from."] # [derive (Debug , serde :: Serialize)] # [serde (rename_all = "snake_case" , tag = "kind")] pub enum PackageFile { # [doc = " File being copied from another location."] Copy { # [doc = " An absolute path to the actual file content"] path : PathBuf , } , # [doc = " File being generated during packaging"] Generate { # [doc = " An absolute path to the original file the generated one is based on."] # [doc = " if any."] # [serde (skip_serializing_if = "Option::is_none")] path : Option < PathBuf > , } , }
};
}
