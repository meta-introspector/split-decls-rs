// Generated macro for WatchDescriptor (struct)
macro_rules! Depcrate_watchesWatchDescriptor {
() => {
// Module: crate::watches
// Provides: {"WatchDescriptor"}
// Dependencies: {}
# [doc = " Represents a watch on an inode"] # [doc = ""] # [doc = " Can be obtained from [`Watches::add`] or from an [`Event`]. A watch"] # [doc = " descriptor can be used to get inotify to stop watching an inode by passing"] # [doc = " it to [`Watches::remove`]."] # [doc = ""] # [doc = " [`Event`]: crate::Event"] # [derive (Clone , Debug)] pub struct WatchDescriptor { pub (crate) id : c_int , pub (crate) fd : Weak < FdGuard > , }
};
}
