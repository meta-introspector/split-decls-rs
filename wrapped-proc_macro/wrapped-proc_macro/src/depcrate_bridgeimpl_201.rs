// Generated macro for impl_201 (impl)
macro_rules! Depcrate_bridgeimpl_201 {
() => {
// Module: crate::bridge
// Provides: {"impl_201"}
// Dependencies: {}
impl < T : Mark > Mark for Vec < T > { type Unmarked = Vec < T :: Unmarked > ; fn mark (unmarked : Self :: Unmarked) -> Self { unmarked . into_iter () . map (T :: mark) . collect () } }
};
}
