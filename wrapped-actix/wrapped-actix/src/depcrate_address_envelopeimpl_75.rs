// Generated macro for impl_75 (impl)
macro_rules! Depcrate_address_envelopeimpl_75 {
() => {
// Module: crate::address::envelope
// Provides: {"impl_75"}
// Dependencies: {}
impl < A : Actor > Envelope < A > { pub fn new < M > (msg : M , tx : Option < Sender < M :: Result > >) -> Self where A : Handler < M > , A :: Context : AsyncContext < A > , M : Message + Send + 'static , M :: Result : Send , { Envelope (Box :: new (SyncEnvelopeProxy { tx , msg : Some (msg) })) } pub fn with_proxy (proxy : Box < dyn EnvelopeProxy < A > + Send >) -> Self { Envelope (proxy) } }
};
}
