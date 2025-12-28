macro_rules! deps {
    () => {
        Fuse!();
    };
}

macro_rules! PanicFuseConsumer {
    () => {
        deps!();
        struct PanicFuseConsumer < 'a , C > { base : C , fuse : Fuse < 'a > , }
    };
}

PanicFuseConsumer!();