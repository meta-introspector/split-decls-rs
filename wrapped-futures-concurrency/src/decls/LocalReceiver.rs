macro_rules! deps {
    () => {
        LocalChannel!();
    };
}

macro_rules! LocalReceiver {
    () => {
        deps!();
        pub (crate) struct LocalReceiver < T > { channel : Rc < RefCell < LocalChannel < T > > > , }
    };
}

LocalReceiver!();