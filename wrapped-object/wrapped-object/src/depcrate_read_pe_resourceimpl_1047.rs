// Generated macro for impl_1047 (impl)
macro_rules! Depcrate_read_pe_resourceimpl_1047 {
() => {
// Module: crate::read::pe::resource
// Provides: {"impl_1047"}
// Dependencies: {}
impl < 'data > ResourceDirectoryEntryData < 'data > { # [doc = " Converts to an option of table."] # [doc = ""] # [doc = " Helper for iterator filtering."] pub fn table (self) -> Option < ResourceDirectoryTable < 'data > > { match self { Self :: Table (dir) => Some (dir) , _ => None , } } # [doc = " Converts to an option of data entry."] # [doc = ""] # [doc = " Helper for iterator filtering."] pub fn data (self) -> Option < & 'data pe :: ImageResourceDataEntry > { match self { Self :: Data (rsc) => Some (rsc) , _ => None , } } }
};
}
