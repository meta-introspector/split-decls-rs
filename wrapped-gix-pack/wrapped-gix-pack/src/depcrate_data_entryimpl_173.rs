// Generated macro for impl_173 (impl)
macro_rules! Depcrate_data_entryimpl_173 {
() => {
// Module: crate::data::entry
// Provides: {"impl_173"}
// Dependencies: {}
impl Location { # [doc = " Compute a range suitable for lookup in pack data using the [`entry_slice()`][crate::data::File::entry_slice()] method."] pub fn entry_range (& self , pack_offset : data :: Offset) -> crate :: data :: EntryRange { pack_offset .. pack_offset + self . entry_size as u64 } }
};
}
