// Generated macro for impl_403 (impl)
macro_rules! Depcrate_re_unicodeimpl_403 {
() => {
// Module: crate::re_unicode
// Provides: {"impl_403"}
// Dependencies: {}
impl < 'r , 't > Iterator for FindCaptures < 'r , 't > { type Item = Captures < 't > ; fn next (& mut self) -> Option < Captures < 't > > { match self . 0 { FindCapturesInner :: Dynamic (ref mut it) => { let named = it . regex () . capture_name_idx () . clone () ; it . next () . map (| slots | Captures { text : it . text () , slots : slots , named_groups : NamedGroups :: Dynamic (named) , }) } FindCapturesInner :: Plugin (ref mut it) => { it . next () . map (| slots | Captures { text : it . text () , slots : slots , named_groups : NamedGroups :: Plugin (it . regex () . groups) , }) } } } }
};
}
