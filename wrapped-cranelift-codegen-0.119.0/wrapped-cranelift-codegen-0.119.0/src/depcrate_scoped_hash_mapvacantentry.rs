// Generated macro for VacantEntry (struct)
macro_rules! Depcrate_scoped_hash_mapVacantEntry {
() => {
// Module: crate::scoped_hash_map
// Provides: {"VacantEntry"}
// Dependencies: {}
# [doc = " A view into a vacant entry in a `ScopedHashMap`. It is part of the `Entry` enum."] pub struct VacantEntry < 'a , K : 'a , V : 'a > { entry : InsertLoc < 'a , K , V > , depth : u32 , generation : u32 , }
};
}
