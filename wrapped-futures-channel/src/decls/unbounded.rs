macro_rules! deps {
    () => {
        UnboundedReceiver!();
        UnboundedSender!();
        UnboundedSenderInner!();
        UnboundedInner!();
        Queue!();
    };
}

macro_rules! unbounded {
    () => {
        deps!();
        # [doc = " Creates an unbounded mpsc channel for communicating between asynchronous"] # [doc = " tasks."] # [doc = ""] # [doc = " A `send` on this channel will always succeed as long as the receive half has"] # [doc = " not been closed. If the receiver falls behind, messages will be arbitrarily"] # [doc = " buffered."] # [doc = ""] # [doc = " **Note** that the amount of available system memory is an implicit bound to"] # [doc = " the channel. Using an `unbounded` channel has the ability of causing the"] # [doc = " process to run out of memory. In this case, the process will be aborted."] pub fn unbounded < T > () -> (UnboundedSender < T > , UnboundedReceiver < T >) { let inner = Arc :: new (UnboundedInner { state : AtomicUsize :: new (INIT_STATE) , message_queue : Queue :: new () , num_senders : AtomicUsize :: new (1) , recv_task : AtomicWaker :: new () , }) ; let tx = UnboundedSenderInner { inner : inner . clone () } ; let rx = UnboundedReceiver { inner : Some (inner) } ; (UnboundedSender (Some (tx)) , rx) }
    };
}

unbounded!();