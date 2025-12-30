// Generated macro for impl_73 (impl)
macro_rules! Depcrate_address_envelopeimpl_73 {
() => {
// Module: crate::address::envelope
// Provides: {"impl_73"}
// Dependencies: {}
impl < A , M > ToEnvelope < A , M > for Context < A > where A : Actor < Context = Context < A > > + Handler < M > , M : Message + Send + 'static , M :: Result : Send , { fn pack (msg : M , tx : Option < Sender < M :: Result > >) -> Envelope < A > { Envelope :: new (msg , tx) } }
};
}
