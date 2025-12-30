// Generated macro for VacantEntryRef (struct)
macro_rules! Depcrate_mapref_entry_refVacantEntryRef {
() => {
// Module: crate::mapref::entry_ref
// Provides: {"VacantEntryRef"}
// Dependencies: {}
pub struct VacantEntryRef < 'a , 'q , K , Q , V > { shard : RwLockWriteGuardDetached < 'a > , entry : hash_table :: VacantEntry < 'a , (K , V) > , key : & 'q Q , }
};
}
