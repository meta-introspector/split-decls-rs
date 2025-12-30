// Generated macro for MatchingReceiver (trait)
macro_rules! Depcrate_channelMatchingReceiver {
() => {
// Module: crate::channel
// Provides: {"MatchingReceiver"}
// Dependencies: {}
# [doc = " Abstraction over different connections that receive data"] pub trait MatchingReceiver { # [doc = " Type of callback"] type F ; # [doc = " Add a callback to be called in case a message matches."] # [doc = ""] # [doc = " Returns an id that can be used to remove the callback."] fn start_receive (& self , m : MatchRule < 'static > , f : Self :: F) -> Token ; # [doc = " Remove a previously added callback."] fn stop_receive (& self , id : Token) -> Option < (MatchRule < 'static > , Self :: F) > ; }
};
}
