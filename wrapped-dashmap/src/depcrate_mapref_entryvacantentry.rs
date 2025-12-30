// Generated macro for VacantEntry (struct)
macro_rules! Depcrate_mapref_entryVacantEntry {
() => {
// Module: crate::mapref::entry
// Provides: {"VacantEntry"}
// Dependencies: {}
pub struct VacantEntry < 'a , K , V > { shard : RwLockWriteGuardDetached < 'a > , key : K , entry : hash_table :: VacantEntry < 'a , (K , V) > , }
};
}
