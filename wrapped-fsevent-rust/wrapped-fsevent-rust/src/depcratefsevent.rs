// Generated macro for FsEvent (struct)
macro_rules! DepcrateFsEvent {
() => {
// Module: crate
// Provides: {"FsEvent"}
// Dependencies: {}
pub struct FsEvent { paths : Vec < String > , since_when : FSEventStreamEventId , latency : CFTimeInterval , flags : FSEventStreamCreateFlags , runloop : Option < CFRetained < CFRunLoop > > , }
};
}
