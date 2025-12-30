// Generated macro for count_new_keys (function)
macro_rules! Depcrate_unique_implcount_new_keys {
() => {
// Module: crate::unique_impl
// Provides: {"count_new_keys"}
// Dependencies: {}
fn count_new_keys < I , K > (mut used : HashMap < K , () > , iterable : I) -> usize where I : IntoIterator < Item = K > , K : Hash + Eq , { let iter = iterable . into_iter () ; let current_used = used . len () ; used . extend (iter . map (| key | (key , ()))) ; used . len () - current_used }
};
}
