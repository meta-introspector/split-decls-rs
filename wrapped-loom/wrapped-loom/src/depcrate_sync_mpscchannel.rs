// Generated macro for channel (function)
macro_rules! Depcrate_sync_mpscchannel {
() => {
// Module: crate::sync::mpsc
// Provides: {"channel"}
// Dependencies: {}
# [doc = " Mock implementation of `std::sync::mpsc::channel`."] # [track_caller] pub fn channel < T > () -> (Sender < T > , Receiver < T >) { let location = location ! () ; let (sender_channel , receiver_channel) = std :: sync :: mpsc :: channel () ; let channel = std :: sync :: Arc :: new (rt :: Channel :: new (location)) ; let sender = Sender { object : std :: sync :: Arc :: clone (& channel) , sender : sender_channel , } ; let receiver = Receiver { object : std :: sync :: Arc :: clone (& channel) , receiver : receiver_channel , } ; (sender , receiver) }
};
}
