// Generated macro for EntryRef (enum)
macro_rules! Depcrate_mapref_entry_refEntryRef {
() => {
// Module: crate::mapref::entry_ref
// Provides: {"EntryRef"}
// Dependencies: {}
# [doc = " Entry with a borrowed key."] pub enum EntryRef < 'a , 'q , K , Q , V > { Occupied (OccupiedEntryRef < 'a , 'q , K , Q , V >) , Vacant (VacantEntryRef < 'a , 'q , K , Q , V >) , }
};
}
