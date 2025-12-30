// Generated macro for mk_map (function)
macro_rules! Depcrate_testsmk_map {
() => {
// Module: crate::tests
// Provides: {"mk_map"}
// Dependencies: {}
fn mk_map < K : Ord , V > (entries : Vec < (K , V) >) -> BTreeMap < K , V > { BTreeMap :: from_iter (entries . into_iter ()) }
};
}
