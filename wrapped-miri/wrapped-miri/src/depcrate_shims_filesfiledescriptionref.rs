// Generated macro for FileDescriptionRef (struct)
macro_rules! Depcrate_shims_filesFileDescriptionRef {
() => {
// Module: crate::shims::files
// Provides: {"FileDescriptionRef"}
// Dependencies: {}
# [doc = " A refcounted pointer to a file description, also tracking the"] # [doc = " globally unique ID of this file description."] # [repr (transparent)] # [derive (CoercePointee , Debug)] pub struct FileDescriptionRef < T : ? Sized > (Rc < FdIdWith < T > >) ;
};
}
