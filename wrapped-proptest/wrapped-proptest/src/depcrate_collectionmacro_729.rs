// Generated macro for macro_729 (macro)
macro_rules! Depcrate_collectionmacro_729 {
() => {
// Module: crate::collection
// Provides: {"macro_729"}
// Dependencies: {}
mapfn ! { [] fn VecToBTreeSet [< T : fmt :: Debug + Ord >] (vec : Vec < T >) -> BTreeSet < T > { vec . into_iter () . collect () } }
};
}
