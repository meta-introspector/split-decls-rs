// Generated macro for impl_874 (impl)
macro_rules! Depcrate_read_macho_symbolimpl_874 {
() => {
// Module: crate::read::macho::symbol
// Provides: {"impl_874"}
// Dependencies: {}
impl < 'data , 'file , Mach , R > fmt :: Debug for MachOSymbolIterator < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MachOSymbolIterator") . finish () } }
};
}
