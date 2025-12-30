// Generated macro for impl_212 (impl)
macro_rules! Depcrate_ots_signatureimpl_212 {
() => {
// Module: crate::ots::signature
// Provides: {"impl_212"}
// Dependencies: {}
impl < Mode : LmsOtsMode > SignatureEncoding for Signature < Mode > where Mode :: PLen : Add < U1 > , Mode :: NLen : Mul < Sum < Mode :: PLen , U1 > > , Prod < Mode :: NLen , Sum < Mode :: PLen , U1 > > : Add < U4 > , Sum < Prod < Mode :: NLen , Sum < Mode :: PLen , U1 > > , U4 > : ArraySize , { type Repr = Output < Mode > ; }
};
}
