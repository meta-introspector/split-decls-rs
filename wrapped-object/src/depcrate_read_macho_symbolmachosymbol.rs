// Generated macro for MachOSymbol (struct)
macro_rules! Depcrate_read_macho_symbolMachOSymbol {
() => {
// Module: crate::read::macho::symbol
// Provides: {"MachOSymbol"}
// Dependencies: {}
# [doc = " A symbol in a [`MachOFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSymbol`] trait implementation."] # [derive (Debug , Clone , Copy)] pub struct MachOSymbol < 'data , 'file , Mach , R = & 'data [u8] > where Mach : MachHeader , R : ReadRef < 'data > , { file : & 'file MachOFile < 'data , Mach , R > , index : SymbolIndex , nlist : & 'data Mach :: Nlist , }
};
}
