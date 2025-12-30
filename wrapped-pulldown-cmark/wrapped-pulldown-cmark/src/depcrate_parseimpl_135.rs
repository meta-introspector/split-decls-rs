// Generated macro for impl_135 (impl)
macro_rules! Depcrate_parseimpl_135 {
() => {
// Module: crate::parse
// Provides: {"impl_135"}
// Dependencies: {}
impl < 'a , F : BrokenLinkCallback < 'a > > Iterator for Parser < 'a , F > { type Item = Event < 'a > ; fn next (& mut self) -> Option < Event < 'a > > { let broken_link_callback = self . broken_link_callback . as_mut () . map (| f | f as & mut dyn BrokenLinkCallback < 'a >) ; self . inner . next_event_range (broken_link_callback) . map (| (event , _range) | event) } }
};
}
