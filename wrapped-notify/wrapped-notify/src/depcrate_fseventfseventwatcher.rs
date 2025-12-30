// Generated macro for FsEventWatcher (struct)
macro_rules! Depcrate_fseventFsEventWatcher {
() => {
// Module: crate::fsevent
// Provides: {"FsEventWatcher"}
// Dependencies: {}
# [doc = " FSEvents-based `Watcher` implementation"] pub struct FsEventWatcher { paths : cf :: CFMutableArrayRef , since_when : fs :: FSEventStreamEventId , latency : cf :: CFTimeInterval , flags : fs :: FSEventStreamCreateFlags , event_handler : Arc < Mutex < dyn EventHandler > > , runloop : Option < (cf :: CFRunLoopRef , thread :: JoinHandle < () >) > , recursive_info : HashMap < PathBuf , bool > , }
};
}
