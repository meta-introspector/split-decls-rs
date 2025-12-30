// Generated macro for Writable (struct)
macro_rules! Depcrate_reactorWritable {
() => {
// Module: crate::reactor
// Provides: {"Writable"}
// Dependencies: {}
# [doc = " Future for [`Async::writable`](crate::Async::writable)."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Writable < 'a , T > (Ready < & 'a crate :: Async < T > , T >) ;
};
}
