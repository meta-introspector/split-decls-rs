// Generated macro for MachOFile (struct)
macro_rules! Depcrate_read_macho_fileMachOFile {
() => {
// Module: crate::read::macho::file
// Provides: {"MachOFile"}
// Dependencies: {}
# [doc = " A partially parsed Mach-O file."] # [doc = ""] # [doc = " Most of the functionality of this type is provided by the [`Object`] trait implementation."] # [derive (Debug)] pub struct MachOFile < 'data , Mach , R = & 'data [u8] > where Mach : MachHeader , R : ReadRef < 'data > , { pub (super) endian : Mach :: Endian , pub (super) data : R , pub (super) header_offset : u64 , pub (super) header : & 'data Mach , pub (super) segments : Vec < MachOSegmentInternal < 'data , Mach , R > > , pub (super) sections : Vec < MachOSectionInternal < 'data , Mach , R > > , pub (super) symbols : SymbolTable < 'data , Mach , R > , }
};
}
