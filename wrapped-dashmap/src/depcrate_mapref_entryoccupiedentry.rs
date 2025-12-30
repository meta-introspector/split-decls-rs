// Generated macro for OccupiedEntry (struct)
macro_rules! Depcrate_mapref_entryOccupiedEntry {
() => {
// Module: crate::mapref::entry
// Provides: {"OccupiedEntry"}
// Dependencies: {}
pub struct OccupiedEntry < 'a , K , V > { shard : RwLockWriteGuardDetached < 'a > , entry : hash_table :: OccupiedEntry < 'a , (K , V) > , key : K , }
};
}
