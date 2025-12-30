// Generated macro for impl_190 (impl)
macro_rules! Depcrate_ots_publicimpl_190 {
() => {
// Module: crate::ots::public
// Provides: {"impl_190"}
// Dependencies: {}
# [doc = " Converts a [`VerifyingKey`] into its byte representation"] impl < Mode : LmsOtsMode > From < VerifyingKey < Mode > > for Array < u8 , Sum < < Mode :: Hasher as OutputSizeUser > :: OutputSize , U24 > > where < Mode :: Hasher as OutputSizeUser > :: OutputSize : Add < U24 > , Sum < < Mode :: Hasher as OutputSizeUser > :: OutputSize , U24 > : ArraySize , { fn from (pk : VerifyingKey < Mode >) -> Self { Array :: try_from_iter (std :: iter :: empty () . chain (Mode :: TYPECODE . to_be_bytes ()) . chain (pk . id) . chain (pk . q . to_be_bytes ()) . chain (pk . k) ,) . expect ("ok") } }
};
}
