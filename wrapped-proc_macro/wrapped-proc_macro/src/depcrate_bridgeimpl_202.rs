// Generated macro for impl_202 (impl)
macro_rules! Depcrate_bridgeimpl_202 {
() => {
// Module: crate::bridge
// Provides: {"impl_202"}
// Dependencies: {}
impl < T : Unmark > Unmark for Vec < T > { type Unmarked = Vec < T :: Unmarked > ; fn unmark (self) -> Self :: Unmarked { self . into_iter () . map (T :: unmark) . collect () } }
};
}
