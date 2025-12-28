macro_rules! deps {
    () => {
        Receiver!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        # [doc = " A blocking iterator over messages in a channel."] # [doc = ""] # [doc = " Each call to [`next`] blocks waiting for the next message and then returns it. However, if the"] # [doc = " channel becomes empty and disconnected, it returns [`None`] without blocking."] # [doc = ""] # [doc = " [`next`]: Iterator::next"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::thread;"] # [doc = " use crossbeam_channel::unbounded;"] # [doc = ""] # [doc = " let (s, r) = unbounded();"] # [doc = ""] # [doc = " thread::spawn(move || {"] # [doc = "     s.send(1).unwrap();"] # [doc = "     s.send(2).unwrap();"] # [doc = "     s.send(3).unwrap();"] # [doc = "     drop(s); // Disconnect the channel."] # [doc = " });"] # [doc = ""] # [doc = " // Collect all messages from the channel."] # [doc = " // Note that the call to `collect` blocks until the sender is dropped."] # [doc = " let v: Vec<_> = r.iter().collect();"] # [doc = ""] # [doc = " assert_eq!(v, [1, 2, 3]);"] # [doc = " ```"] pub struct Iter < 'a , T > { receiver : & 'a Receiver < T > , }
    };
}

Iter!()