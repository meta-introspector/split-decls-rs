// Generated macro for macro_737 (macro)
macro_rules! Depcrate_collectionmacro_737 {
() => {
// Module: crate::collection
// Provides: {"macro_737"}
// Dependencies: {}
mapfn ! { [] fn VecToBTreeMap [< K : fmt :: Debug + Ord , V : fmt :: Debug >] (vec : Vec < (K , V) >) -> BTreeMap < K , V > { vec . into_iter () . collect () } }
};
}
