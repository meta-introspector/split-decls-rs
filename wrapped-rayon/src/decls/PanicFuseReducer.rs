macro_rules! deps {
    () => {
        Fuse!();
    };
}

macro_rules! PanicFuseReducer {
    () => {
        deps!();
        struct PanicFuseReducer < 'a , C > { base : C , _fuse : Fuse < 'a > , }
    };
}

PanicFuseReducer!();