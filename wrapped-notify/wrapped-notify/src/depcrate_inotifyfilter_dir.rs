// Generated macro for filter_dir (function)
macro_rules! Depcrate_inotifyfilter_dir {
() => {
// Module: crate::inotify
// Provides: {"filter_dir"}
// Dependencies: {}
# [doc = " return `DirEntry` when it is a directory"] fn filter_dir (e : walkdir :: Result < walkdir :: DirEntry >) -> Option < walkdir :: DirEntry > { if let Ok (e) = e { if let Ok (metadata) = e . metadata () { if metadata . is_dir () { return Some (e) ; } } } None }
};
}
