// Generated macro for ReadDirectoryChangesServer (struct)
macro_rules! Depcrate_windowsReadDirectoryChangesServer {
() => {
// Module: crate::windows
// Provides: {"ReadDirectoryChangesServer"}
// Dependencies: {}
struct ReadDirectoryChangesServer { tx : Sender < Action > , rx : Receiver < Action > , event_handler : Arc < Mutex < dyn EventHandler > > , meta_tx : Sender < MetaEvent > , cmd_tx : Sender < Result < PathBuf > > , watches : HashMap < PathBuf , WatchState > , wakeup_sem : HANDLE , }
};
}
