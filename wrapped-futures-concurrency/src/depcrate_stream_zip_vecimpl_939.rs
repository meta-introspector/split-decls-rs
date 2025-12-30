// Generated macro for impl_939 (impl)
macro_rules! Depcrate_stream_zip_vecimpl_939 {
() => {
// Module: crate::stream::zip::vec
// Provides: {"impl_939"}
// Dependencies: {}
impl < S > fmt :: Debug for Zip < S > where S : Stream + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . streams . iter ()) . finish () } }
};
}
