// Generated macro for impl_1986 (impl)
macro_rules! Depcrate_sink_with_flat_mapimpl_1986 {
() => {
// Module: crate::sink::with_flat_map
// Provides: {"impl_1986"}
// Dependencies: {}
impl < Si , Item , U , St , F > WithFlatMap < Si , Item , U , St , F > where Si : Sink < Item > , F : FnMut (U) -> St , St : Stream < Item = Result < Item , Si :: Error > > , { pub (super) fn new (sink : Si , f : F) -> Self { Self { sink , f , stream : None , buffer : None , _marker : PhantomData } } delegate_access_inner ! (sink , Si , ()) ; fn try_empty_stream (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Si :: Error > > { let mut this = self . project () ; if this . buffer . is_some () { ready ! (this . sink . as_mut () . poll_ready (cx)) ? ; let item = this . buffer . take () . unwrap () ; this . sink . as_mut () . start_send (item) ? ; } if let Some (mut some_stream) = this . stream . as_mut () . as_pin_mut () { while let Some (item) = ready ! (some_stream . as_mut () . poll_next (cx) ?) { match this . sink . as_mut () . poll_ready (cx) ? { Poll :: Ready (()) => this . sink . as_mut () . start_send (item) ? , Poll :: Pending => { * this . buffer = Some (item) ; return Poll :: Pending ; } } ; } } this . stream . set (None) ; Poll :: Ready (Ok (())) } }
};
}
