// Generated macro for impl_896 (impl)
macro_rules! Depcrate_read_macho_relocationimpl_896 {
() => {
// Module: crate::read::macho::relocation
// Provides: {"impl_896"}
// Dependencies: {}
impl < 'data , 'file , Mach , R > fmt :: Debug for MachORelocationIterator < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MachORelocationIterator") . finish () } }
};
}
