// Generated macro for OccupiedEntry (struct)
macro_rules! Depcrate_ctxhashOccupiedEntry {
() => {
// Module: crate::ctxhash
// Provides: {"OccupiedEntry"}
// Dependencies: {}
# [doc = " A reference to an occupied entry in the hash table."] pub struct OccupiedEntry < 'a , K , V > { raw : hashbrown :: hash_table :: OccupiedEntry < 'a , BucketData < K , V > > , }
};
}
