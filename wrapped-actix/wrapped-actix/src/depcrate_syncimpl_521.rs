// Generated macro for impl_521 (impl)
macro_rules! Depcrate_syncimpl_521 {
() => {
// Module: crate::sync
// Provides: {"impl_521"}
// Dependencies: {}
impl < A , M > EnvelopeProxy < A > for SyncContextEnvelope < M > where M : Message + Send + 'static , M :: Result : Send , A : Actor < Context = SyncContext < A > > + Handler < M > , { fn handle (& mut self , act : & mut A , ctx : & mut A :: Context) { let tx = self . tx . take () ; if tx . is_some () && tx . as_ref () . unwrap () . is_closed () { return ; } if let Some (msg) = self . msg . take () { < A as Handler < M > > :: handle (act , msg , ctx) . handle (ctx , tx) } } }
};
}
