// Generated macro for impl_54 (impl)
macro_rules! Depcrate_lms_privateimpl_54 {
() => {
// Module: crate::lms::private
// Provides: {"impl_54"}
// Dependencies: {}
# [doc = " Converts a [PrivateKey] into its byte representation"] impl < Mode : LmsMode > From < SigningKey < Mode > > for Array < u8 , Sum < < Mode :: Hasher as OutputSizeUser > :: OutputSize , U28 > > where < Mode :: Hasher as OutputSizeUser > :: OutputSize : Add < U28 > , Sum < < Mode :: Hasher as OutputSizeUser > :: OutputSize , U28 > : ArraySize , { fn from (pk : SigningKey < Mode >) -> Self { Array :: try_from_iter (std :: iter :: empty () . chain (Mode :: TYPECODE . to_be_bytes ()) . chain (Mode :: OtsMode :: TYPECODE . to_be_bytes ()) . chain (pk . q . to_be_bytes ()) . chain (pk . id) . chain (pk . seed) ,) . unwrap () } }
};
}
