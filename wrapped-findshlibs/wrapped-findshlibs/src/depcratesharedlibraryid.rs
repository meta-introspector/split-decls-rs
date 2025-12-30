// Generated macro for SharedLibraryId (enum)
macro_rules! DepcrateSharedLibraryId {
() => {
// Module: crate
// Provides: {"SharedLibraryId"}
// Dependencies: {}
# [doc = " Represents an ID for a shared library."] # [derive (PartialEq , Eq , Hash)] pub enum SharedLibraryId { # [doc = " A UUID (used on mac)"] Uuid ([u8 ; 16]) , # [doc = " A GNU build ID"] GnuBuildId (Vec < u8 >) , # [doc = " The PE timestamp and size"] PeSignature (u32 , u32) , # [doc = " A PDB GUID and age,"] PdbSignature ([u8 ; 16] , u32) , }
};
}
