// Generated macro for impl_606 (impl)
macro_rules! Depcrate_frame_dataimpl_606 {
() => {
// Module: crate::frame::data
// Provides: {"impl_606"}
// Dependencies: {}
impl < T : Buf > Data < T > { # [doc = " Encode the data frame into the `dst` buffer."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `dst` cannot contain the data frame."] pub (crate) fn encode_chunk < U : BufMut > (& mut self , dst : & mut U) { let len = self . data . remaining () ; assert ! (dst . remaining_mut () >= len) ; self . head () . encode (len , dst) ; dst . put (& mut self . data) ; } }
};
}
