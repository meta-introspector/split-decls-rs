// Generated macro for impl_595 (impl)
macro_rules! Depcrate_read_lookupimpl_595 {
() => {
// Module: crate::read::lookup
// Provides: {"impl_595"}
// Dependencies: {}
impl < R , Parser > DebugLookup < R , Parser > where R : Reader , Parser : LookupParser < R > , { pub fn items (& self) -> LookupEntryIter < R , Parser > { LookupEntryIter { current_set : None , remaining_input : self . input_buffer . clone () , } } pub fn reader (& self) -> & R { & self . input_buffer } }
};
}
