// Generated macro for VacantEntry (struct)
macro_rules! Depcrate_ctxhashVacantEntry {
() => {
// Module: crate::ctxhash
// Provides: {"VacantEntry"}
// Dependencies: {}
# [doc = " A reference to a vacant entry in the hash table."] pub struct VacantEntry < 'a , K , V > { hash : u32 , key : K , raw : hashbrown :: hash_table :: VacantEntry < 'a , BucketData < K , V > > , }
};
}
