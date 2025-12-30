// Generated macro for Accept (struct)
macro_rules! Depcrate_acceptAccept {
() => {
// Module: crate::accept
// Provides: {"Accept"}
// Dependencies: {}
# [doc = " Poll instance of the server."] pub (crate) struct Accept { poll : Poll , waker_queue : WakerQueue , handles : Vec < WorkerHandleAccept > , srv : ServerHandle , next : usize , avail : Availability , # [doc = " use the smallest duration from sockets timeout."] timeout : Option < Duration > , paused : bool , }
};
}
