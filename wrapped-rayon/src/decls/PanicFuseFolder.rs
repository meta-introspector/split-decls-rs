macro_rules! deps {
    () => {
        Fuse!();
    };
}

macro_rules! PanicFuseFolder {
    () => {
        deps!();
        struct PanicFuseFolder < 'a , C > { base : C , fuse : Fuse < 'a > , }
    };
}

PanicFuseFolder!();