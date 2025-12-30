// Generated macro for WeakFileDescriptionRef (struct)
macro_rules! Depcrate_shims_filesWeakFileDescriptionRef {
() => {
// Module: crate::shims::files
// Provides: {"WeakFileDescriptionRef"}
// Dependencies: {}
# [doc = " Holds a weak reference to the actual file description."] # [derive (Debug)] pub struct WeakFileDescriptionRef < T : ? Sized > (Weak < FdIdWith < T > >) ;
};
}
