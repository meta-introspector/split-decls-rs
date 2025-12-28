macro_rules! deps {
    () => {
        Any!();
        Out!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl Out { unsafe fn new < T > (t : T) -> Self { Out (unsafe { Any :: new (t) }) } unsafe fn take < T > (self) -> T { unsafe { self . 0 . take () } } }
    };
}

impl_23!()