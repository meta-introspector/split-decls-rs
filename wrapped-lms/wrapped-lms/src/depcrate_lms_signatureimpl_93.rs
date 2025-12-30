// Generated macro for impl_93 (impl)
macro_rules! Depcrate_lms_signatureimpl_93 {
() => {
// Module: crate::lms::signature
// Provides: {"impl_93"}
// Dependencies: {}
impl < Mode : LmsMode > From < Signature < Mode > > for Vec < u8 > where < Mode :: OtsMode as LmsOtsMode > :: PLen : Add < U1 > , < Mode :: OtsMode as LmsOtsMode > :: NLen : Mul < Sum < < Mode :: OtsMode as LmsOtsMode > :: PLen , U1 > > , Prod < < Mode :: OtsMode as LmsOtsMode > :: NLen , Sum < < Mode :: OtsMode as LmsOtsMode > :: PLen , U1 > > : Add < U4 > , Sum < Prod < < Mode :: OtsMode as LmsOtsMode > :: NLen , Sum < < Mode :: OtsMode as LmsOtsMode > :: PLen , U1 > > , U4 , > : ArraySize , { fn from (val : Signature < Mode >) -> Self { let mut sig = Vec :: new () ; sig . extend_from_slice (& val . q . to_be_bytes ()) ; let lms_sig : Array < u8 , _ > = val . lmots_sig . into () ; sig . extend_from_slice (& lms_sig) ; sig . extend_from_slice (& Mode :: TYPECODE . to_be_bytes ()) ; for node in val . path { sig . extend_from_slice (& node) ; } sig } }
};
}
