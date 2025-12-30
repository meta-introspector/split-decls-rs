// Generated macro for channel (function)
macro_rules! Depcrate_address_channelchannel {
() => {
// Module: crate::address::channel
// Provides: {"channel"}
// Dependencies: {}
# [doc = " Creates an in-memory channel implementation of the `Stream` trait with"] # [doc = " bounded capacity."] # [doc = ""] # [doc = " This method creates a concrete implementation of the `Stream` trait which"] # [doc = " can be used to send values across threads in a streaming fashion. This"] # [doc = " channel is unique in that it implements back pressure to ensure that the"] # [doc = " sender never outpaces the receiver. The channel capacity is equal to"] # [doc = " `buffer + num-senders`. In other words, each sender gets a guaranteed slot"] # [doc = " in the channel capacity, and on top of that there are `buffer` \"first come,"] # [doc = " first serve\" slots available to all senders."] # [doc = ""] # [doc = " The `Receiver` returned implements the `Stream` trait and has access to any"] # [doc = " number of the associated combinators for transforming the result."] pub fn channel < A : Actor > (buffer : usize) -> (AddressSender < A > , AddressReceiver < A >) { assert ! (buffer < MAX_BUFFER , "requested buffer size too large") ; let inner = Arc :: new (Inner { buffer : AtomicUsize :: new (buffer) , state : AtomicUsize :: new (INIT_STATE) , message_queue : Queue :: new () , parked_queue : Queue :: new () , num_senders : AtomicUsize :: new (1) , recv_task : AtomicWaker :: new () , }) ; let tx = AddressSender { inner : Arc :: clone (& inner) , sender_task : Arc :: new (Mutex :: new (SenderTask :: new ())) , maybe_parked : Arc :: new (AtomicBool :: new (false)) , } ; let rx = AddressReceiver { inner } ; (tx , rx) }
};
}
