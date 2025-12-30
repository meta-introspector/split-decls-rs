// Generated macro for TryIter (struct)
macro_rules! Depcrate_channelTryIter {
() => {
// Module: crate::channel
// Provides: {"TryIter"}
// Dependencies: {}
# [doc = " A non-blocking iterator over messages in a channel."] # [doc = ""] # [doc = " Each call to [`next`] returns a message if there is one ready to be received. The iterator"] # [doc = " never blocks waiting for the next message."] # [doc = ""] # [doc = " [`next`]: Iterator::next"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::thread;"] # [doc = " use std::time::Duration;"] # [doc = " use crossbeam_channel::unbounded;"] # [doc = ""] # [doc = " let (s, r) = unbounded::<i32>();"] # [doc = ""] # [doc = " # let t ="] # [doc = " thread::spawn(move || {"] # [doc = "     s.send(1).unwrap();"] # [doc = "     thread::sleep(Duration::from_secs(1));"] # [doc = "     s.send(2).unwrap();"] # [doc = "     thread::sleep(Duration::from_secs(2));"] # [doc = "     s.send(3).unwrap();"] # [doc = " });"] # [doc = ""] # [doc = " thread::sleep(Duration::from_secs(2));"] # [doc = ""] # [doc = " // Collect all messages from the channel without blocking."] # [doc = " // The third message hasn't been sent yet so we'll collect only the first two."] # [doc = " let v: Vec<_> = r.try_iter().collect();"] # [doc = ""] # [doc = " assert_eq!(v, [1, 2]);"] # [doc = " # t.join().unwrap(); // join thread to avoid https://github.com/rust-lang/miri/issues/1371"] # [doc = " ```"] pub struct TryIter < 'a , T > { receiver : & 'a Receiver < T > , }
};
}
