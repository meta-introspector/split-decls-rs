macro_rules! deps {
    () => {
        UnboundedInner!();
    };
}

macro_rules! UnboundedSenderInner {
    () => {
        deps!();
        struct UnboundedSenderInner < T > { inner : Arc < UnboundedInner < T > > , }
    };
}

UnboundedSenderInner!()