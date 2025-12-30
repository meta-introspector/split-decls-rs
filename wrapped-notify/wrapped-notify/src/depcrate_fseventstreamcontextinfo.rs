// Generated macro for StreamContextInfo (struct)
macro_rules! Depcrate_fseventStreamContextInfo {
() => {
// Module: crate::fsevent
// Provides: {"StreamContextInfo"}
// Dependencies: {}
struct StreamContextInfo { event_handler : Arc < Mutex < dyn EventHandler > > , recursive_info : HashMap < PathBuf , bool > , }
};
}
