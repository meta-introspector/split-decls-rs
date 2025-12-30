// Generated macro for EventLoop (struct)
macro_rules! Depcrate_inotifyEventLoop {
() => {
// Module: crate::inotify
// Provides: {"EventLoop"}
// Dependencies: {}
struct EventLoop { running : bool , poll : mio :: Poll , event_loop_waker : Arc < mio :: Waker > , event_loop_tx : Sender < EventLoopMsg > , event_loop_rx : Receiver < EventLoopMsg > , inotify : Option < Inotify > , event_handler : Box < dyn EventHandler > , # [doc = " PathBuf -> (WatchDescriptor, WatchMask, is_recursive, is_dir)"] watches : HashMap < PathBuf , (WatchDescriptor , WatchMask , bool , bool) > , paths : HashMap < WatchDescriptor , PathBuf > , rename_event : Option < Event > , follow_links : bool , }
};
}
