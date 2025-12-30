// Generated macro for impl_873 (impl)
macro_rules! Depcrate_read_macho_symbolimpl_873 {
() => {
// Module: crate::read::macho::symbol
// Provides: {"impl_873"}
// Dependencies: {}
impl < 'data , 'file , Mach , R > MachOSymbolIterator < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { pub (super) fn new (file : & 'file MachOFile < 'data , Mach , R >) -> Self { MachOSymbolIterator { file , index : SymbolIndex (0) , } } pub (super) fn empty (file : & 'file MachOFile < 'data , Mach , R >) -> Self { MachOSymbolIterator { file , index : SymbolIndex (file . symbols . len ()) , } } }
};
}
