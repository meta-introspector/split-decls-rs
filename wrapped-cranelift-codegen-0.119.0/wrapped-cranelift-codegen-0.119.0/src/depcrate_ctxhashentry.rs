// Generated macro for Entry (enum)
macro_rules! Depcrate_ctxhashEntry {
() => {
// Module: crate::ctxhash
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " A reference to an existing or vacant entry in the hash table."] pub enum Entry < 'a , K , V > { Occupied (OccupiedEntry < 'a , K , V >) , Vacant (VacantEntry < 'a , K , V >) , }
};
}
