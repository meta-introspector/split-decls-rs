// Generated macro for impl_1214 (impl)
macro_rules! Depcrate_read_xcoff_relocationimpl_1214 {
() => {
// Module: crate::read::xcoff::relocation
// Provides: {"impl_1214"}
// Dependencies: {}
impl < 'data , 'file , Xcoff , R > fmt :: Debug for XcoffRelocationIterator < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("XcoffRelocationIterator") . finish () } }
};
}
