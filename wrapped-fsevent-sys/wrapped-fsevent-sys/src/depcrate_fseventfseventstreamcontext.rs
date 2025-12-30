// Generated macro for FSEventStreamContext (struct)
macro_rules! Depcrate_fseventFSEventStreamContext {
() => {
// Module: crate::fsevent
// Provides: {"FSEventStreamContext"}
// Dependencies: {}
# [repr (C)] pub struct FSEventStreamContext { pub version : CFIndex , pub info : * mut c_void , pub retain : Option < CFAllocatorRetainCallBack > , pub release : Option < CFAllocatorReleaseCallBack > , pub copy_description : Option < CFAllocatorCopyDescriptionCallBack > , }
};
}
