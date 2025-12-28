macro_rules! deps {
    () => {
        Receiver!();
        ReceiverFlavor!();
        Channel!();
        Sender!();
        SenderFlavor!();
    };
}

macro_rules! bounded {
    () => {
        deps!();
        # [doc = " Creates a multi-producer multi-consumer channel of bounded capacity."] # [doc = ""] # [doc = " This channel has a buffer that can hold at most `cap` messages at a time."] # [doc = ""] # [doc = " A special case is zero-capacity channel, which cannot hold any messages. Instead, send and"] # [doc = " receive operations must appear at the same time in order to pair up and pass the message over."] # [doc = ""] # [doc = " For more info on how to use the channel see [module level documentation](index.html)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " A channel of capacity 1:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::thread;"] # [doc = " use std::time::Duration;"] # [doc = " use crossbeam_channel::bounded;"] # [doc = ""] # [doc = " let (s, r) = bounded(1);"] # [doc = ""] # [doc = " // This call returns immediately because there is enough space in the channel."] # [doc = " s.send(1).unwrap();"] # [doc = ""] # [doc = " thread::spawn(move || {"] # [doc = "     // This call blocks the current thread because the channel is full."] # [doc = "     // It will be able to complete only after the first message is received."] # [doc = "     s.send(2).unwrap();"] # [doc = " });"] # [doc = ""] # [doc = " thread::sleep(Duration::from_secs(1));"] # [doc = " assert_eq!(r.recv(), Ok(1));"] # [doc = " assert_eq!(r.recv(), Ok(2));"] # [doc = " ```"] # [doc = ""] # [doc = " A zero-capacity channel:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::thread;"] # [doc = " use std::time::Duration;"] # [doc = " use crossbeam_channel::bounded;"] # [doc = ""] # [doc = " let (s, r) = bounded(0);"] # [doc = ""] # [doc = " thread::spawn(move || {"] # [doc = "     // This call blocks the current thread until a receive operation appears"] # [doc = "     // on the other side of the channel."] # [doc = "     s.send(1).unwrap();"] # [doc = " });"] # [doc = ""] # [doc = " thread::sleep(Duration::from_secs(1));"] # [doc = " assert_eq!(r.recv(), Ok(1));"] # [doc = " ```"] pub fn bounded < T > (cap : usize) -> (Sender < T > , Receiver < T >) { if cap == 0 { let (s , r) = counter :: new (flavors :: zero :: Channel :: new ()) ; let s = Sender { flavor : SenderFlavor :: Zero (s) , } ; let r = Receiver { flavor : ReceiverFlavor :: Zero (r) , } ; (s , r) } else { let (s , r) = counter :: new (flavors :: array :: Channel :: with_capacity (cap)) ; let s = Sender { flavor : SenderFlavor :: Array (s) , } ; let r = Receiver { flavor : ReceiverFlavor :: Array (r) , } ; (s , r) } }
    };
}

bounded!();