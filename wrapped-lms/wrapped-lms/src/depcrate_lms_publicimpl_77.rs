// Generated macro for impl_77 (impl)
macro_rules! Depcrate_lms_publicimpl_77 {
() => {
// Module: crate::lms::public
// Provides: {"impl_77"}
// Dependencies: {}
# [doc = " Converts a [`VerifyingKey`] into its byte representation"] impl < Mode : LmsMode > From < VerifyingKey < Mode > > for Array < u8 , Sum < < Mode :: Hasher as OutputSizeUser > :: OutputSize , U24 > > where < Mode :: Hasher as OutputSizeUser > :: OutputSize : Add < U24 > , Sum < < Mode :: Hasher as OutputSizeUser > :: OutputSize , U24 > : ArraySize , { fn from (pk : VerifyingKey < Mode >) -> Self { Array :: try_from_iter (std :: iter :: empty () . chain (Mode :: TYPECODE . to_be_bytes ()) . chain (Mode :: OtsMode :: TYPECODE . to_be_bytes ()) . chain (pk . id) . chain (pk . k) ,) . unwrap () } }
};
}
