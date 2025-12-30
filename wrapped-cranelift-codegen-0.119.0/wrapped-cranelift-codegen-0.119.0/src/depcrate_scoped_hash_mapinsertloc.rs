// Generated macro for InsertLoc (enum)
macro_rules! Depcrate_scoped_hash_mapInsertLoc {
() => {
// Module: crate::scoped_hash_map
// Provides: {"InsertLoc"}
// Dependencies: {}
# [doc = " Where to insert from a `VacantEntry`. May be vacant or occupied in"] # [doc = " the underlying map because of lazy (generation-based) deletion."] enum InsertLoc < 'a , K : 'a , V : 'a > { Vacant (crate :: ctxhash :: VacantEntry < 'a , K , Val < V > >) , Occupied (crate :: ctxhash :: OccupiedEntry < 'a , K , Val < V > >) , }
};
}
