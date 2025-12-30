// Generated macro for ReadDirectoryChangesWatcher (struct)
macro_rules! Depcrate_windowsReadDirectoryChangesWatcher {
() => {
// Module: crate::windows
// Provides: {"ReadDirectoryChangesWatcher"}
// Dependencies: {}
# [doc = " Watcher implementation based on ReadDirectoryChanges"] # [derive (Debug)] pub struct ReadDirectoryChangesWatcher { tx : Sender < Action > , cmd_rx : Receiver < Result < PathBuf > > , wakeup_sem : HANDLE , }
};
}
