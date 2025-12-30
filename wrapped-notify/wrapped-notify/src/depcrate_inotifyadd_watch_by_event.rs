// Generated macro for add_watch_by_event (function)
macro_rules! Depcrate_inotifyadd_watch_by_event {
() => {
// Module: crate::inotify
// Provides: {"add_watch_by_event"}
// Dependencies: {}
# [inline] fn add_watch_by_event (path : & PathBuf , event : & inotify_sys :: Event < & OsStr > , watches : & HashMap < PathBuf , (WatchDescriptor , WatchMask , bool , bool) > , add_watches : & mut Vec < PathBuf > ,) { if event . mask . contains (EventMask :: ISDIR) { if let Some (parent_path) = path . parent () { if let Some (& (_ , _ , is_recursive , _)) = watches . get (parent_path) { if is_recursive { add_watches . push (path . to_owned ()) ; } } } } }
};
}
