// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl < Dirent > Iterator for DirentIterator < Dirent > { type Item = * mut Dirent ; fn next (& mut self) -> Option < Self :: Item > { if self . index >= self . entries . len () { return None ; } let ptr = & mut self . entries [self . index] ; self . index += 1 ; Some (ptr) } }
};
}
