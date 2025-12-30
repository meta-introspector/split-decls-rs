// Generated macro for impl_897 (impl)
macro_rules! Depcrate_stream_zip_arrayimpl_897 {
() => {
// Module: crate::stream::zip::array
// Provides: {"impl_897"}
// Dependencies: {}
impl < S , const N : usize > fmt :: Debug for Zip < S , N > where S : Stream + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . streams . iter ()) . finish () } }
};
}
