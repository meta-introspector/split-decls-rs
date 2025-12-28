macro_rules! deps {
    () => {
        WakerToHandle!();
        NotifyWaker!();
    };
}

macro_rules! impl_1024 {
    () => {
        deps!();
        impl From < WakerToHandle < '_ > > for NotifyHandle01 { fn from (handle : WakerToHandle < '_ >) -> Self { let ptr = Box :: new (NotifyWaker (handle . 0 . clone ())) ; unsafe { Self :: new (Box :: into_raw (ptr)) } } }
    };
}

impl_1024!()