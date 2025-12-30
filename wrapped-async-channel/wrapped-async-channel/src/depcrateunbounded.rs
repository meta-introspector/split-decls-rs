// Generated macro for unbounded (function)
macro_rules! Depcrateunbounded {
() => {
// Module: crate
// Provides: {"unbounded"}
// Dependencies: {}
# [doc = " Creates an unbounded channel."] # [doc = ""] # [doc = " The created channel can hold an unlimited number of messages."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " use async_channel::{unbounded, TryRecvError};"] # [doc = ""] # [doc = " let (s, r) = unbounded();"] # [doc = ""] # [doc = " assert_eq!(s.send(10).await, Ok(()));"] # [doc = " assert_eq!(s.send(20).await, Ok(()));"] # [doc = ""] # [doc = " assert_eq!(r.recv().await, Ok(10));"] # [doc = " assert_eq!(r.recv().await, Ok(20));"] # [doc = " assert_eq!(r.try_recv(), Err(TryRecvError::Empty));"] # [doc = " # });"] # [doc = " ```"] pub fn unbounded < T > () -> (Sender < T > , Receiver < T >) { let channel = Arc :: new (Channel { queue : ConcurrentQueue :: unbounded () , send_ops : Event :: new () , recv_ops : Event :: new () , stream_ops : Event :: new () , closed_ops : Event :: new () , sender_count : AtomicUsize :: new (1) , receiver_count : AtomicUsize :: new (1) , }) ; let s = Sender { channel : channel . clone () , } ; let r = Receiver { listener : None , channel , _pin : PhantomPinned , } ; (s , r) }
};
}
