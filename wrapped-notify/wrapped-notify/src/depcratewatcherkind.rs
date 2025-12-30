// Generated macro for WatcherKind (enum)
macro_rules! DepcrateWatcherKind {
() => {
// Module: crate
// Provides: {"WatcherKind"}
// Dependencies: {}
# [doc = " Watcher kind enumeration"] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum WatcherKind { # [doc = " inotify backend (linux)"] Inotify , # [doc = " FS-Event backend (mac)"] Fsevent , # [doc = " KQueue backend (bsd,optionally mac)"] Kqueue , # [doc = " Polling based backend (fallback)"] PollWatcher , # [doc = " Windows backend"] ReadDirectoryChangesWatcher , # [doc = " Fake watcher for testing"] NullWatcher , }
};
}
