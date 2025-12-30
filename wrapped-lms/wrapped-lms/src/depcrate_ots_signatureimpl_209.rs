// Generated macro for impl_209 (impl)
macro_rules! Depcrate_ots_signatureimpl_209 {
() => {
// Module: crate::ots::signature
// Provides: {"impl_209"}
// Dependencies: {}
# [doc = " Converts a [`Signature`] into its byte representation"] impl < Mode : LmsOtsMode > From < Signature < Mode > > for Output < Mode > where Mode :: PLen : Add < U1 > , Mode :: NLen : Mul < Sum < Mode :: PLen , U1 > > , Prod < Mode :: NLen , Sum < Mode :: PLen , U1 > > : Add < U4 > , Sum < Prod < Mode :: NLen , Sum < Mode :: PLen , U1 > > , U4 > : ArraySize , { fn from (sig : Signature < Mode >) -> Self { Array :: try_from_iter (std :: iter :: empty () . chain (Mode :: TYPECODE . to_be_bytes ()) . chain (sig . c . clone ()) . chain (sig . y . iter () . flatten () . cloned ()) ,) . expect ("ok") } }
};
}
