// Generated macro for RawOccupiedEntryMut (struct)
macro_rules! Depcrate_linked_hash_mapRawOccupiedEntryMut {
() => {
// Module: crate::linked_hash_map
// Provides: {"RawOccupiedEntryMut"}
// Dependencies: {}
pub struct RawOccupiedEntryMut < 'a , K , V , S > { hash_builder : & 'a S , free : & 'a mut Option < NonNull < Node < K , V > > > , values : & 'a mut Option < NonNull < Node < K , V > > > , entry : hash_table :: OccupiedEntry < 'a , NonNull < Node < K , V > > > , }
};
}
