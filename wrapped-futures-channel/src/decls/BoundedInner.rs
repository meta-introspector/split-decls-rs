macro_rules! deps {
    () => {
        Queue!();
        SenderTask!();
    };
}

macro_rules! BoundedInner {
    () => {
        deps!();
        struct BoundedInner < T > { buffer : usize , state : AtomicUsize , message_queue : Queue < T > , parked_queue : Queue < Arc < Mutex < SenderTask > > > , num_senders : AtomicUsize , recv_task : AtomicWaker , }
    };
}

BoundedInner!();