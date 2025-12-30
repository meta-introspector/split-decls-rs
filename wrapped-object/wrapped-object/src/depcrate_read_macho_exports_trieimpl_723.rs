// Generated macro for impl_723 (impl)
macro_rules! Depcrate_read_macho_exports_trieimpl_723 {
() => {
// Module: crate::read::macho::exports_trie
// Provides: {"impl_723"}
// Dependencies: {}
impl < 'data > Iterator for ExportsTrieIterator < 'data > { type Item = Result < ExportSymbol < 'data > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
};
}
