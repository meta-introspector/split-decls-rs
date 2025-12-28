macro_rules! deps {
    () => {
        Fuse!();
    };
}

macro_rules! PanicFuseIter {
    () => {
        deps!();
        struct PanicFuseIter < 'a , I > { base : I , fuse : Fuse < 'a > , }
    };
}

PanicFuseIter!()