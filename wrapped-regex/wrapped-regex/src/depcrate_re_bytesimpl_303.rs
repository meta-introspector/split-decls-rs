// Generated macro for impl_303 (impl)
macro_rules! Depcrate_re_bytesimpl_303 {
() => {
// Module: crate::re_bytes
// Provides: {"impl_303"}
// Dependencies: {}
impl < 'r , 't > Iterator for FindCaptures < 'r , 't > { type Item = Captures < 't > ; fn next (& mut self) -> Option < Captures < 't > > { self . 0 . next () . map (| slots | Captures { text : self . 0 . text () , slots : slots , named_groups : self . 0 . regex () . capture_name_idx () . clone () , }) } }
};
}
