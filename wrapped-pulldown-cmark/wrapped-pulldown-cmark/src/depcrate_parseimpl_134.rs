// Generated macro for impl_134 (impl)
macro_rules! Depcrate_parseimpl_134 {
() => {
// Module: crate::parse
// Provides: {"impl_134"}
// Dependencies: {}
impl < 'a , F : BrokenLinkCallback < 'a > > Iterator for OffsetIter < 'a , F > { type Item = (Event < 'a > , Range < usize >) ; fn next (& mut self) -> Option < Self :: Item > { let broken_link_callback = self . parser . broken_link_callback . as_mut () . map (| f | f as & mut dyn BrokenLinkCallback < 'a >) ; self . parser . inner . next_event_range (broken_link_callback) } }
};
}
