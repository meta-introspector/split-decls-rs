// Generated macro for Entry (struct)
macro_rules! Depcrate_file_indexEntry {
() => {
// Module: crate::file::index
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " An entry of a chunk file index"] pub struct Entry { # [doc = " The kind of the chunk file"] pub kind : crate :: Id , # [doc = " The offset, relative to the beginning of the file, at which to find the chunk and its end."] pub offset : Range < crate :: file :: Offset > , }
};
}
