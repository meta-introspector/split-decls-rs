// Generated macro for section_from_rva (function)
macro_rules! Depcrate_reader_filesection_from_rva {
() => {
// Module: crate::reader::file
// Provides: {"section_from_rva"}
// Dependencies: {}
fn section_from_rva (sections : & [IMAGE_SECTION_HEADER] , rva : u32) -> Option < & IMAGE_SECTION_HEADER > { sections . iter () . find (| & s | { rva >= s . VirtualAddress && rva < s . VirtualAddress + unsafe { s . Misc . VirtualSize } }) }
};
}
