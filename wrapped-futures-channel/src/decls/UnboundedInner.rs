macro_rules! deps {
    () => {
        Queue!();
    };
}

macro_rules! UnboundedInner {
    () => {
        deps!();
        struct UnboundedInner < T > { state : AtomicUsize , message_queue : Queue < T > , num_senders : AtomicUsize , recv_task : AtomicWaker , }
    };
}

UnboundedInner!();