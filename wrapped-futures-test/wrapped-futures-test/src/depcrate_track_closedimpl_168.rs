// Generated macro for impl_168 (impl)
macro_rules! Depcrate_track_closedimpl_168 {
() => {
// Module: crate::track_closed
// Provides: {"impl_168"}
// Dependencies: {}
impl < Item , T : Sink < Item > > Sink < Item > for TrackClosed < T > { type Error = T :: Error ; fn poll_ready (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { assert ! (! self . is_closed ()) ; self . project () . inner . poll_ready (cx) } fn start_send (self : Pin < & mut Self > , item : Item) -> Result < () , Self :: Error > { assert ! (! self . is_closed ()) ; self . project () . inner . start_send (item) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { assert ! (! self . is_closed ()) ; self . project () . inner . poll_flush (cx) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { assert ! (! self . is_closed ()) ; let this = self . project () ; match this . inner . poll_close (cx) { Poll :: Ready (Ok (())) => { * this . closed = true ; Poll :: Ready (Ok (())) } other => other , } } }
};
}
