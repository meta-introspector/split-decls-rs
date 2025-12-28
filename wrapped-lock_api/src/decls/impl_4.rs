macro_rules! deps {
    () => {
        GuardNoSend!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        unsafe impl Sync for GuardNoSend { }
    };
}

impl_4!()