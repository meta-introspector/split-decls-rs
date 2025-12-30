// Generated macro for impl_863 (impl)
macro_rules! Depcrate_read_macho_symbolimpl_863 {
() => {
// Module: crate::read::macho::symbol
// Provides: {"impl_863"}
// Dependencies: {}
impl < 'data , Mach : MachHeader , R : ReadRef < 'data > > Default for SymbolTable < 'data , Mach , R > { fn default () -> Self { SymbolTable { symbols : & [] , strings : Default :: default () , } } }
};
}
