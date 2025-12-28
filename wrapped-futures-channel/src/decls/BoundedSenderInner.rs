macro_rules! deps {
    () => {
        BoundedInner!();
        SenderTask!();
    };
}

macro_rules! BoundedSenderInner {
    () => {
        deps!();
        struct BoundedSenderInner < T > { inner : Arc < BoundedInner < T > > , sender_task : Arc < Mutex < SenderTask > > , maybe_parked : bool , }
    };
}

BoundedSenderInner!();