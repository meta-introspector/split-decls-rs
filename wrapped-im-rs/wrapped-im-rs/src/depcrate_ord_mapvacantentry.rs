// Generated macro for VacantEntry (struct)
macro_rules! Depcrate_ord_mapVacantEntry {
() => {
// Module: crate::ord::map
// Provides: {"VacantEntry"}
// Dependencies: {}
# [doc = " An entry for a mapping that does not already exist in the map."] pub struct VacantEntry < 'a , K , V > where K : Ord + Clone , V : Clone , { map : & 'a mut OrdMap < K , V > , key : K , }
};
}
