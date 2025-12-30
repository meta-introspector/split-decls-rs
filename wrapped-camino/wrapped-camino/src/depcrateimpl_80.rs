// Generated macro for impl_80 (impl)
macro_rules! Depcrateimpl_80 {
() => {
// Module: crate
// Provides: {"impl_80"}
// Dependencies: {}
impl Iterator for ReadDirUtf8 { type Item = io :: Result < Utf8DirEntry > ; fn next (& mut self) -> Option < io :: Result < Utf8DirEntry > > { self . inner . next () . map (| entry | entry . and_then (Utf8DirEntry :: new)) } }
};
}
