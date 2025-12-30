// Generated macro for ResourceDirectoryTable (struct)
macro_rules! Depcrate_read_pe_resourceResourceDirectoryTable {
() => {
// Module: crate::read::pe::resource
// Provides: {"ResourceDirectoryTable"}
// Dependencies: {}
# [doc = " A table of resource entries."] # [derive (Debug , Clone)] pub struct ResourceDirectoryTable < 'data > { # [doc = " The table header."] pub header : & 'data pe :: ImageResourceDirectory , # [doc = " The table entries."] pub entries : & 'data [pe :: ImageResourceDirectoryEntry] , }
};
}
