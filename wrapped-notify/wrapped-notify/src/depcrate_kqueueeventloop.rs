// Generated macro for EventLoop (struct)
macro_rules! Depcrate_kqueueEventLoop {
() => {
// Module: crate::kqueue
// Provides: {"EventLoop"}
// Dependencies: {}
struct EventLoop { running : bool , poll : mio :: Poll , event_loop_waker : Arc < mio :: Waker > , event_loop_tx : Sender < EventLoopMsg > , event_loop_rx : Receiver < EventLoopMsg > , kqueue : kqueue :: Watcher , event_handler : Box < dyn EventHandler > , watches : HashMap < PathBuf , bool > , follow_symlinks : bool , }
};
}
