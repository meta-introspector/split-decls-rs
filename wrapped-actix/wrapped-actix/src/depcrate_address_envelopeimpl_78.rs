// Generated macro for impl_78 (impl)
macro_rules! Depcrate_address_envelopeimpl_78 {
() => {
// Module: crate::address::envelope
// Provides: {"impl_78"}
// Dependencies: {}
impl < A , M > EnvelopeProxy < A > for SyncEnvelopeProxy < M > where M : Message + Send + 'static , M :: Result : Send , A : Actor + Handler < M > , A :: Context : AsyncContext < A > , { fn handle (& mut self , act : & mut A , ctx : & mut < A as Actor > :: Context) { let tx = self . tx . take () ; if tx . is_some () && tx . as_ref () . unwrap () . is_closed () { return ; } if let Some (msg) = self . msg . take () { let fut = < A as Handler < M > > :: handle (act , msg , ctx) ; fut . handle (ctx , tx) } } }
};
}
