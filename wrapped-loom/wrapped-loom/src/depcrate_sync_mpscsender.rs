// Generated macro for Sender (struct)
macro_rules! Depcrate_sync_mpscSender {
() => {
// Module: crate::sync::mpsc
// Provides: {"Sender"}
// Dependencies: {}
# [derive (Debug)] # [doc = " Mock implementation of `std::sync::mpsc::Sender`."] pub struct Sender < T > { object : std :: sync :: Arc < rt :: Channel > , sender : std :: sync :: mpsc :: Sender < T > , }
};
}
