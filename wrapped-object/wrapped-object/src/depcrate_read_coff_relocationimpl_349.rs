// Generated macro for impl_349 (impl)
macro_rules! Depcrate_read_coff_relocationimpl_349 {
() => {
// Module: crate::read::coff::relocation
// Provides: {"impl_349"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > fmt :: Debug for CoffRelocationIterator < 'data , 'file , R , Coff > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("CoffRelocationIterator") . finish () } }
};
}
