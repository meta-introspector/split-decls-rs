macro_rules! deps {
    () => {
        Fuse!();
    };
}

macro_rules! PanicFuseProducer {
    () => {
        deps!();
        struct PanicFuseProducer < 'a , P > { base : P , fuse : Fuse < 'a > , }
    };
}

PanicFuseProducer!();