// Generated macro for LookupEntryIter (struct)
macro_rules! Depcrate_read_lookupLookupEntryIter {
() => {
// Module: crate::read::lookup
// Provides: {"LookupEntryIter"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct LookupEntryIter < R , Parser > where R : Reader , Parser : LookupParser < R > , { current_set : Option < (R , Parser :: Header) > , remaining_input : R , }
};
}
