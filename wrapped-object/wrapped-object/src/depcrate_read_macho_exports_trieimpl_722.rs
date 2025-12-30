// Generated macro for impl_722 (impl)
macro_rules! Depcrate_read_macho_exports_trieimpl_722 {
() => {
// Module: crate::read::macho::exports_trie
// Provides: {"impl_722"}
// Dependencies: {}
impl < 'data > ExportsTrieIterator < 'data > { pub (super) fn new (data : & 'data [u8]) -> Self { ExportsTrieIterator { node_iter : NodeIterator :: new (data) , } } # [doc = " Returns the next exported symbol, if any."] fn next (& mut self) -> Result < Option < ExportSymbol < 'data > > > { for node in & mut self . node_iter { if let Some (export_symbol) = node ? { return Ok (Some (export_symbol)) ; } } Ok (None) } }
};
}
