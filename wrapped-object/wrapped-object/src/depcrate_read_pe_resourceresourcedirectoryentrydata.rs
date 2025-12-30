// Generated macro for ResourceDirectoryEntryData (enum)
macro_rules! Depcrate_read_pe_resourceResourceDirectoryEntryData {
() => {
// Module: crate::read::pe::resource
// Provides: {"ResourceDirectoryEntryData"}
// Dependencies: {}
# [doc = " Data associated with a resource directory entry."] # [derive (Debug , Clone)] pub enum ResourceDirectoryEntryData < 'data > { # [doc = " A subtable entry."] Table (ResourceDirectoryTable < 'data >) , # [doc = " A resource data entry."] Data (& 'data pe :: ImageResourceDataEntry) , }
};
}
