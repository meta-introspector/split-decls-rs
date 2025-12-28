macro_rules! deps {
    () => {
        LocalChannel!();
    };
}

macro_rules! LocalSender {
    () => {
        deps!();
        pub (crate) struct LocalSender < T > { channel : Rc < RefCell < LocalChannel < T > > > , }
    };
}

LocalSender!()