// Generated macro for impl_729 (impl)
macro_rules! Depcrate_read_macho_exports_trieimpl_729 {
() => {
// Module: crate::read::macho::exports_trie
// Provides: {"impl_729"}
// Dependencies: {}
impl < 'data > Iterator for NodeIterator < 'data > { type Item = Result < Option < ExportSymbol < 'data > > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
};
}
