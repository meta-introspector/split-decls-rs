macro_rules! deps {
    () => {
        LocalSender!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < T > LocalSender < T > { pub (crate) fn send (& self , item : T) { let mut channel = self . channel . borrow_mut () ; channel . queue . push_back (item) ; let _ = channel . waker . take () . map (Waker :: wake) ; } }
    };
}

impl_120!();