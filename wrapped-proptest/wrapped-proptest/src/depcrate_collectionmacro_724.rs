// Generated macro for macro_724 (macro)
macro_rules! Depcrate_collectionmacro_724 {
() => {
// Module: crate::collection
// Provides: {"macro_724"}
// Dependencies: {}
mapfn ! { { # [cfg (feature = "std")] } [] fn VecToHashSet [< T : fmt :: Debug + Hash + Eq >] (vec : Vec < T >) -> HashSet < T > { vec . into_iter () . collect () } }
};
}
