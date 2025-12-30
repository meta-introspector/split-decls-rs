// Generated macro for impl_875 (impl)
macro_rules! Depcrate_read_macho_symbolimpl_875 {
() => {
// Module: crate::read::macho::symbol
// Provides: {"impl_875"}
// Dependencies: {}
impl < 'data , 'file , Mach , R > Iterator for MachOSymbolIterator < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { type Item = MachOSymbol < 'data , 'file , Mach , R > ; fn next (& mut self) -> Option < Self :: Item > { loop { let index = self . index ; let nlist = self . file . symbols . symbols . get (index . 0) ? ; self . index . 0 += 1 ; if let Some (symbol) = MachOSymbol :: new (self . file , index , nlist) { return Some (symbol) ; } } } }
};
}
