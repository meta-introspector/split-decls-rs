// Generated macro for impl_75 (impl)
macro_rules! Depcrateimpl_75 {
() => {
// Module: crate
// Provides: {"impl_75"}
// Dependencies: {}
impl < T > Iterator for Indices < '_ , T > { type Item = Index < T > ; fn next (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . head . map (| index | { let entry = self . entries [index . get ()] . occupied_ref () ; let index = Index :: new (index , entry . generation) ; self . head = entry . next ; self . remaining -= 1 ; index }) } } fn size_hint (& self) -> (usize , Option < usize >) { (self . remaining , Some (self . remaining)) } }
};
}
