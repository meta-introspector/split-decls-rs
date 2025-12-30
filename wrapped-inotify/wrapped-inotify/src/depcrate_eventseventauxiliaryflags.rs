// Generated macro for EventAuxiliaryFlags (struct)
macro_rules! Depcrate_eventsEventAuxiliaryFlags {
() => {
// Module: crate::events
// Provides: {"EventAuxiliaryFlags"}
// Dependencies: {}
# [doc = " Auxiliary flags for inotify events"] # [doc = ""] # [doc = " The non-mutually-exclusive bitflags that may be set"] # [doc = " in an event read from an inotify fd. 0 or more of these"] # [doc = " bitflags may be set."] # [derive (Debug , Clone , Copy , Hash , PartialEq , Eq , Default)] pub struct EventAuxiliaryFlags { # [doc = " Watch was removed when explicitly removed via [`inotify_rm_watch(2)`]"] # [doc = " or automatically (because the file was deleted or the filesystem was unmounted)"] # [doc = ""] # [doc = " [`inotify_rm_watch(2)`]: https://man7.org/linux/man-pages/man2/inotify_rm_watch.2.html"] pub ignored : bool , # [doc = " Event subject is a directory rather than a regular file"] pub isdir : bool , # [doc = " File system containing watched object was unmounted"] # [doc = ""] # [doc = " An event with **IN_IGNORED** will subsequently be generated for the same watch descriptor."] pub unmount : bool , }
};
}
