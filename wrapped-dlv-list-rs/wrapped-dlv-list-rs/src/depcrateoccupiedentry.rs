// Generated macro for OccupiedEntry (struct)
macro_rules! DepcrateOccupiedEntry {
() => {
// Module: crate
// Provides: {"OccupiedEntry"}
// Dependencies: {}
# [doc = " An occupied entry in the list."] # [derive (Clone)] struct OccupiedEntry < T > { # [doc = " The generation of when this entry was inserted. This is used to avoid the ABA problem."] generation : u64 , # [doc = " The index of the next occupied entry in the list."] next : Option < NonMaxUsize > , # [doc = " The index of the previous occupied entry in the list."] previous : Option < NonMaxUsize > , # [doc = " The actual value being stored in this entry."] value : T , }
};
}
