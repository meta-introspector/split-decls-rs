// Generated macro for RawVacantEntryMut (struct)
macro_rules! Depcrate_linked_hash_mapRawVacantEntryMut {
() => {
// Module: crate::linked_hash_map
// Provides: {"RawVacantEntryMut"}
// Dependencies: {}
pub struct RawVacantEntryMut < 'a , K , V , S > { hash_builder : & 'a S , values : & 'a mut Option < NonNull < Node < K , V > > > , free : & 'a mut Option < NonNull < Node < K , V > > > , entry : hash_table :: AbsentEntry < 'a , NonNull < Node < K , V > > > , }
};
}
