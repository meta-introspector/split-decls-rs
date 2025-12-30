// Generated macro for Name (enum)
macro_rules! Depcrate_write_coff_writerName {
() => {
// Module: crate::write::coff::writer
// Provides: {"Name"}
// Dependencies: {}
# [doc = " A section or symbol name."] # [derive (Debug , Clone , Copy)] pub enum Name { # [doc = " An inline name."] Short ([u8 ; 8]) , # [doc = " An id of a string table entry."] Long (StringId) , }
};
}
