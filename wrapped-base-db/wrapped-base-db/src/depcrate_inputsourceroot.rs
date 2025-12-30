// Generated macro for SourceRoot (struct)
macro_rules! Depcrate_inputSourceRoot {
() => {
// Module: crate::input
// Provides: {"SourceRoot"}
// Dependencies: {}
# [doc = " Files are grouped into source roots. A source root is a directory on the"] # [doc = " file systems which is watched for changes. Typically it corresponds to a"] # [doc = " Rust crate. Source roots *might* be nested: in this case, a file belongs to"] # [doc = " the nearest enclosing source root. Paths to files are always relative to a"] # [doc = " source root, and the analyzer does not know the root path of the source root at"] # [doc = " all. So, a file from one source root can't refer to a file in another source"] # [doc = " root by path."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct SourceRoot { # [doc = " Sysroot or crates.io library."] # [doc = ""] # [doc = " Libraries are considered mostly immutable, this assumption is used to"] # [doc = " optimize salsa's query structure"] pub is_library : bool , file_set : FileSet , }
};
}
