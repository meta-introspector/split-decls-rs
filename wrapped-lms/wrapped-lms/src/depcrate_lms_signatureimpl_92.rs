// Generated macro for impl_92 (impl)
macro_rules! Depcrate_lms_signatureimpl_92 {
() => {
// Module: crate::lms::signature
// Provides: {"impl_92"}
// Dependencies: {}
impl < Mode : LmsMode > SignatureEncoding for Signature < Mode > where < Mode :: OtsMode as LmsOtsMode > :: PLen : Add < U1 > , < Mode :: OtsMode as LmsOtsMode > :: NLen : Mul < Sum < < Mode :: OtsMode as LmsOtsMode > :: PLen , U1 > > , Prod < < Mode :: OtsMode as LmsOtsMode > :: NLen , Sum < < Mode :: OtsMode as LmsOtsMode > :: PLen , U1 > > : Add < U4 > , Sum < Prod < < Mode :: OtsMode as LmsOtsMode > :: NLen , Sum < < Mode :: OtsMode as LmsOtsMode > :: PLen , U1 > > , U4 , > : ArraySize , { type Repr = Vec < u8 > ; }
};
}
