// Generated macro for channel (function)
macro_rules! Depcrate_mpscchannel {
() => {
// Module: crate::mpsc
// Provides: {"channel"}
// Dependencies: {}
# [doc = " Creates a bounded mpsc channel for communicating between asynchronous tasks."] # [doc = ""] # [doc = " Being bounded, this channel provides backpressure to ensure that the sender"] # [doc = " outpaces the receiver by only a limited amount. The channel's capacity is"] # [doc = " equal to `buffer + num-senders`. In other words, each sender gets a"] # [doc = " guaranteed slot in the channel capacity, and on top of that there are"] # [doc = " `buffer` \"first come, first serve\" slots available to all senders."] # [doc = ""] # [doc = " The [`Receiver`] returned implements the [`Stream`] trait, while [`Sender`]"] # [doc = " implements `Sink`."] pub fn channel < T > (buffer : usize) -> (Sender < T > , Receiver < T >) { assert ! (buffer < MAX_BUFFER , "requested buffer size too large") ; let inner = Arc :: new (BoundedInner { buffer , state : AtomicUsize :: new (INIT_STATE) , message_queue : Queue :: new () , parked_queue : Queue :: new () , num_senders : AtomicUsize :: new (1) , recv_task : AtomicWaker :: new () , }) ; let tx = BoundedSenderInner { inner : inner . clone () , sender_task : Arc :: new (Mutex :: new (SenderTask :: new ())) , maybe_parked : false , } ; let rx = Receiver { inner : Some (inner) } ; (Sender (Some (tx)) , rx) }
};
}
