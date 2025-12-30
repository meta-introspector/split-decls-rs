// Generated macro for offset_from_rva (function)
macro_rules! Depcrate_winmd_fileoffset_from_rva {
() => {
// Module: crate::winmd::file
// Provides: {"offset_from_rva"}
// Dependencies: {}
fn offset_from_rva (section : & IMAGE_SECTION_HEADER , rva : u32) -> usize { (rva - section . VirtualAddress + section . PointerToRawData) as usize }
};
}
