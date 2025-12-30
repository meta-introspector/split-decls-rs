// Generated macro for macro_733 (macro)
macro_rules! Depcrate_collectionmacro_733 {
() => {
// Module: crate::collection
// Provides: {"macro_733"}
// Dependencies: {}
mapfn ! { { # [cfg (feature = "std")] } [] fn VecToHashMap [< K : fmt :: Debug + Hash + Eq , V : fmt :: Debug >] (vec : Vec < (K , V) >) -> HashMap < K , V > { vec . into_iter () . collect () } }
};
}
