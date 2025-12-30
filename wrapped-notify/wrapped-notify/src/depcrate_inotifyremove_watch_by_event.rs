// Generated macro for remove_watch_by_event (function)
macro_rules! Depcrate_inotifyremove_watch_by_event {
() => {
// Module: crate::inotify
// Provides: {"remove_watch_by_event"}
// Dependencies: {}
# [inline] fn remove_watch_by_event (path : & PathBuf , watches : & HashMap < PathBuf , (WatchDescriptor , WatchMask , bool , bool) > , remove_watches : & mut Vec < PathBuf > ,) { if watches . contains_key (path) { remove_watches . push (path . to_owned ()) ; } }
};
}
