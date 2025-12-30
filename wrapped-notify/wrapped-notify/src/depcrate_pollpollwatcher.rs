// Generated macro for PollWatcher (struct)
macro_rules! Depcrate_pollPollWatcher {
() => {
// Module: crate::poll
// Provides: {"PollWatcher"}
// Dependencies: {}
# [doc = " Polling based `Watcher` implementation."] # [doc = ""] # [doc = " By default scans through all files and checks for changed entries based on their change date."] # [doc = " Can also be changed to perform file content change checks."] # [doc = ""] # [doc = " See [Config] for more details."] # [derive (Debug)] pub struct PollWatcher { watches : Arc < Mutex < HashMap < PathBuf , WatchData > > > , data_builder : Arc < Mutex < DataBuilder > > , want_to_stop : Arc < AtomicBool > , # [doc = " channel to the poll loop"] # [doc = " currently used only for manual polling"] message_channel : Sender < () > , delay : Option < Duration > , follow_sylinks : bool , }
};
}
