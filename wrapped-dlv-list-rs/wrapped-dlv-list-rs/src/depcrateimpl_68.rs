// Generated macro for impl_68 (impl)
macro_rules! Depcrateimpl_68 {
() => {
// Module: crate
// Provides: {"impl_68"}
// Dependencies: {}
impl < T > Iterator for Drain < '_ , T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . head . map (| index | { let entry = self . list . remove_entry (index) . expect ("expected occupied entry") ; self . head = entry . next ; self . remaining -= 1 ; entry . value }) } } fn size_hint (& self) -> (usize , Option < usize >) { (self . remaining , Some (self . remaining)) } }
};
}
