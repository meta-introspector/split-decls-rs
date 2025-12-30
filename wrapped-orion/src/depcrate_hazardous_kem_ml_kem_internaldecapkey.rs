// Generated macro for DecapKey (struct)
macro_rules! Depcrate_hazardous_kem_ml_kem_internalDecapKey {
() => {
// Module: crate::hazardous::kem::ml_kem::internal
// Provides: {"DecapKey"}
// Dependencies: {}
pub (crate) struct DecapKey < const K : usize , const ENCODED_SIZE_EK : usize , const ENCODED_SIZE_DK : usize , Pke : PkeParameters , > { pub (crate) bytes : [u8 ; ENCODED_SIZE_DK] , s_hat : [RingElementNTT ; K] , _phantom : PhantomData < Pke > , }
};
}
