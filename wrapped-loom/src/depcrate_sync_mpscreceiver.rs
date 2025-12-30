// Generated macro for Receiver (struct)
macro_rules! Depcrate_sync_mpscReceiver {
() => {
// Module: crate::sync::mpsc
// Provides: {"Receiver"}
// Dependencies: {}
# [derive (Debug)] # [doc = " Mock implementation of `std::sync::mpsc::Receiver`."] pub struct Receiver < T > { object : std :: sync :: Arc < rt :: Channel > , receiver : std :: sync :: mpsc :: Receiver < T > , }
};
}
