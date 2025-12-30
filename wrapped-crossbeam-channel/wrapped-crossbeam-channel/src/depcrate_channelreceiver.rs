// Generated macro for Receiver (struct)
macro_rules! Depcrate_channelReceiver {
() => {
// Module: crate::channel
// Provides: {"Receiver"}
// Dependencies: {}
# [doc = " The receiving side of a channel."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::thread;"] # [doc = " use std::time::Duration;"] # [doc = " use crossbeam_channel::unbounded;"] # [doc = ""] # [doc = " let (s, r) = unbounded();"] # [doc = ""] # [doc = " thread::spawn(move || {"] # [doc = "     let _ = s.send(1);"] # [doc = "     thread::sleep(Duration::from_secs(1));"] # [doc = "     let _ = s.send(2);"] # [doc = " });"] # [doc = ""] # [doc = " assert_eq!(r.recv(), Ok(1)); // Received immediately."] # [doc = " assert_eq!(r.recv(), Ok(2)); // Received after 1 second."] # [doc = " ```"] pub struct Receiver < T > { flavor : ReceiverFlavor < T > , }
};
}
