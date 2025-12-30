// Generated macro for impl_223 (impl)
macro_rules! Depcrate_data_input_bytes_to_entriesimpl_223 {
() => {
// Module: crate::data::input::bytes_to_entries
// Provides: {"impl_223"}
// Dependencies: {}
impl crate :: data :: File { # [doc = " Returns an iterator over [`Entries`][crate::data::input::Entry], without making use of the memory mapping."] pub fn streaming_iter (& self) -> Result < BytesToEntriesIter < impl io :: BufRead > , input :: Error > { let reader = io :: BufReader :: with_capacity (4096 * 8 , fs :: File :: open (& self . path) . map_err (gix_hash :: io :: Error :: from) ?) ; BytesToEntriesIter :: new_from_header (reader , input :: Mode :: Verify , input :: EntryDataMode :: KeepAndCrc32 , self . object_hash ,) } }
};
}
