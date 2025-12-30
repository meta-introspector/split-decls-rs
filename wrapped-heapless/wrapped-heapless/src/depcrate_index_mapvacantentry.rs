// Generated macro for VacantEntry (struct)
macro_rules! Depcrate_index_mapVacantEntry {
() => {
// Module: crate::index_map
// Provides: {"VacantEntry"}
// Dependencies: {}
# [doc = " A view into an empty slot in the underlying map"] pub struct VacantEntry < 'a , K , V , const N : usize > { key : K , hash_val : HashValue , core : & 'a mut CoreMap < K , V , N > , }
};
}
