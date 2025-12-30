// Generated macro for impl_1941 (impl)
macro_rules! Depcrate_sink_send_allimpl_1941 {
() => {
// Module: crate::sink::send_all
// Provides: {"impl_1941"}
// Dependencies: {}
impl < 'a , Si , St , Ok , Error > SendAll < 'a , Si , St > where Si : Sink < Ok , Error = Error > + Unpin + ? Sized , St : TryStream < Ok = Ok , Error = Error > + Stream , { pub (super) fn new (sink : & 'a mut Si , stream : St) -> Self { Self { sink , stream : stream . fuse () , buffered : None } } fn try_start_send (self : Pin < & mut Self > , cx : & mut Context < '_ > , item : St :: Ok ,) -> Poll < Result < () , Si :: Error > > { let this = self . project () ; debug_assert ! (this . buffered . is_none ()) ; match Pin :: new (& mut * this . sink) . poll_ready (cx) ? { Poll :: Ready (()) => Poll :: Ready (Pin :: new (& mut * this . sink) . start_send (item)) , Poll :: Pending => { * this . buffered = Some (item) ; Poll :: Pending } } } }
};
}
