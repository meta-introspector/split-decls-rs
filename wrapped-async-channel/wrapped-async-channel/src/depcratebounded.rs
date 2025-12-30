// Generated macro for bounded (function)
macro_rules! Depcratebounded {
() => {
// Module: crate
// Provides: {"bounded"}
// Dependencies: {}
# [doc = " Creates a bounded channel."] # [doc = ""] # [doc = " The created channel has space to hold at most `cap` messages at a time."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Capacity must be a positive number. If `cap` is zero, this function will panic."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " use async_channel::{bounded, TryRecvError, TrySendError};"] # [doc = ""] # [doc = " let (s, r) = bounded(1);"] # [doc = ""] # [doc = " assert_eq!(s.send(10).await, Ok(()));"] # [doc = " assert_eq!(s.try_send(20), Err(TrySendError::Full(20)));"] # [doc = ""] # [doc = " assert_eq!(r.recv().await, Ok(10));"] # [doc = " assert_eq!(r.try_recv(), Err(TryRecvError::Empty));"] # [doc = " # });"] # [doc = " ```"] pub fn bounded < T > (cap : usize) -> (Sender < T > , Receiver < T >) { assert ! (cap > 0 , "capacity cannot be zero") ; let channel = Arc :: new (Channel { queue : ConcurrentQueue :: bounded (cap) , send_ops : Event :: new () , recv_ops : Event :: new () , stream_ops : Event :: new () , closed_ops : Event :: new () , sender_count : AtomicUsize :: new (1) , receiver_count : AtomicUsize :: new (1) , }) ; let s = Sender { channel : channel . clone () , } ; let r = Receiver { listener : None , channel , _pin : PhantomPinned , } ; (s , r) }
};
}
