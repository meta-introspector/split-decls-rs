// Generated macro for Readable (struct)
macro_rules! Depcrate_reactorReadable {
() => {
// Module: crate::reactor
// Provides: {"Readable"}
// Dependencies: {}
# [doc = " Future for [`Async::readable`](crate::Async::readable)."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Readable < 'a , T > (Ready < & 'a crate :: Async < T > , T >) ;
};
}
