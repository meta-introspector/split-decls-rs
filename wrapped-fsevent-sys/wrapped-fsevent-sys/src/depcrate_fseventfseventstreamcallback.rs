// Generated macro for FSEventStreamCallback (type)
macro_rules! Depcrate_fseventFSEventStreamCallback {
() => {
// Module: crate::fsevent
// Provides: {"FSEventStreamCallback"}
// Dependencies: {}
pub type FSEventStreamCallback = extern "C" fn (FSEventStreamRef , * mut c_void , usize , * mut c_void , * const FSEventStreamEventFlags , * const FSEventStreamEventId ,) ;
};
}
