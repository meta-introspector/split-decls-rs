// Generated macro for ReadableOwned (struct)
macro_rules! Depcrate_reactorReadableOwned {
() => {
// Module: crate::reactor
// Provides: {"ReadableOwned"}
// Dependencies: {}
# [doc = " Future for [`Async::readable_owned`](crate::Async::readable_owned)."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ReadableOwned < T > (Ready < Arc < crate :: Async < T > > , T >) ;
};
}
