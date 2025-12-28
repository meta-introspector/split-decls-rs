macro_rules! deps {
    () => {
        LocalReceiver!();
        LocalChannel!();
        LocalSender!();
    };
}

macro_rules! local_channel {
    () => {
        deps!();
        pub (crate) fn local_channel < T > () -> (LocalSender < T > , LocalReceiver < T >) { let channel = Rc :: new (RefCell :: new (LocalChannel { queue : VecDeque :: new () , waker : None , closed : false , })) ; (LocalSender { channel : channel . clone () , } , LocalReceiver { channel } ,) }
    };
}

local_channel!()