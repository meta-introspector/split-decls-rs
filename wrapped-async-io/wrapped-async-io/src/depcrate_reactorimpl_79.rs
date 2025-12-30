// Generated macro for impl_79 (impl)
macro_rules! Depcrate_reactorimpl_79 {
() => {
// Module: crate::reactor
// Provides: {"impl_79"}
// Dependencies: {}
impl < H : Borrow < crate :: Async < T > > + Clone , T > Future for Ready < H , T > { type Output = io :: Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let Self { ref handle , dir , ticks , index , .. } = & mut * self ; let mut state = handle . borrow () . source . state . lock () . unwrap () ; if let Some ((a , b)) = * ticks { if state [* dir] . tick != a && state [* dir] . tick != b { return Poll :: Ready (Ok (())) ; } } let was_empty = state [* dir] . is_empty () ; let i = match * index { Some (i) => i , None => { let i = state [* dir] . wakers . insert (None) ; * index = Some (i) ; * ticks = Some ((Reactor :: get () . ticker () , state [* dir] . tick)) ; i } } ; state [* dir] . wakers [i] = Some (cx . waker () . clone ()) ; if was_empty { let event = { let mut event = Event :: none (handle . borrow () . source . key) ; event . readable = ! state [READ] . is_empty () ; event . writable = ! state [WRITE] . is_empty () ; event } ; handle . borrow () . source . registration . modify (& Reactor :: get () . poller , event) ? ; } Poll :: Pending } }
};
}
