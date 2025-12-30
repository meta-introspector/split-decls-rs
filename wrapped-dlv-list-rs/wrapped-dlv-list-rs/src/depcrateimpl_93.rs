// Generated macro for impl_93 (impl)
macro_rules! Depcrateimpl_93 {
() => {
// Module: crate
// Provides: {"impl_93"}
// Dependencies: {}
impl < T > DoubleEndedIterator for IterMut < '_ , T > { fn next_back (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . tail . map (| index | { let entry = unsafe { & mut (* self . entries) [index . get ()] } . occupied_mut () ; self . tail = entry . previous ; self . remaining -= 1 ; & mut entry . value }) } } }
};
}
