// Generated macro for impl_82 (impl)
macro_rules! Depcrateimpl_82 {
() => {
// Module: crate
// Provides: {"impl_82"}
// Dependencies: {}
impl < T > Iterator for IntoIter < T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . head . map (| index | { let entry = self . list . remove_entry (index) . expect ("expected occupied entry") ; self . head = entry . next ; self . remaining -= 1 ; entry . value }) } } fn size_hint (& self) -> (usize , Option < usize >) { (self . remaining , Some (self . remaining)) } }
};
}
