// Generated macro for Sender (struct)
macro_rules! Depcrate_channelSender {
() => {
// Module: crate::channel
// Provides: {"Sender"}
// Dependencies: {}
# [doc = " The sending side of a channel."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::thread;"] # [doc = " use crossbeam_channel::unbounded;"] # [doc = ""] # [doc = " let (s1, r) = unbounded();"] # [doc = " let s2 = s1.clone();"] # [doc = ""] # [doc = " thread::spawn(move || s1.send(1).unwrap());"] # [doc = " thread::spawn(move || s2.send(2).unwrap());"] # [doc = ""] # [doc = " let msg1 = r.recv().unwrap();"] # [doc = " let msg2 = r.recv().unwrap();"] # [doc = ""] # [doc = " assert_eq!(msg1 + msg2, 3);"] # [doc = " ```"] pub struct Sender < T > { flavor : SenderFlavor < T > , }
};
}
