// Generated macro for WritableOwned (struct)
macro_rules! Depcrate_reactorWritableOwned {
() => {
// Module: crate::reactor
// Provides: {"WritableOwned"}
// Dependencies: {}
# [doc = " Future for [`Async::writable_owned`](crate::Async::writable_owned)."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct WritableOwned < T > (Ready < Arc < crate :: Async < T > > , T >) ;
};
}
