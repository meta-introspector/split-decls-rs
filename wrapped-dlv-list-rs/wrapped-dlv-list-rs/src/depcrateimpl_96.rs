// Generated macro for impl_96 (impl)
macro_rules! Depcrateimpl_96 {
() => {
// Module: crate
// Provides: {"impl_96"}
// Dependencies: {}
impl < 'a , T > Iterator for IterMut < 'a , T > { type Item = & 'a mut T ; fn next (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . head . map (| index | { let entry = unsafe { & mut (* self . entries) [index . get ()] } . occupied_mut () ; self . head = entry . next ; self . remaining -= 1 ; & mut entry . value }) } } fn size_hint (& self) -> (usize , Option < usize >) { (self . remaining , Some (self . remaining)) } }
};
}
