// Generated macro for OccupiedEntryRef (struct)
macro_rules! Depcrate_mapref_entry_refOccupiedEntryRef {
() => {
// Module: crate::mapref::entry_ref
// Provides: {"OccupiedEntryRef"}
// Dependencies: {}
pub struct OccupiedEntryRef < 'a , 'q , K , Q , V > { shard : RwLockWriteGuardDetached < 'a > , entry : hash_table :: OccupiedEntry < 'a , (K , V) > , key : & 'q Q , }
};
}
