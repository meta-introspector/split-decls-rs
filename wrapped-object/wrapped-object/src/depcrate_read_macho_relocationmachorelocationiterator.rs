// Generated macro for MachORelocationIterator (struct)
macro_rules! Depcrate_read_macho_relocationMachORelocationIterator {
() => {
// Module: crate::read::macho::relocation
// Provides: {"MachORelocationIterator"}
// Dependencies: {}
# [doc = " An iterator for the relocations in a [`MachOSection`](super::MachOSection)."] pub struct MachORelocationIterator < 'data , 'file , Mach , R = & 'data [u8] > where Mach : MachHeader , R : ReadRef < 'data > , { pub (super) file : & 'file MachOFile < 'data , Mach , R > , pub (super) relocations : slice :: Iter < 'data , macho :: Relocation < Mach :: Endian > > , }
};
}
