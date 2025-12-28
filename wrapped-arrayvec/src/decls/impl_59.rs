macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        unsafe impl < 'a , T : Sync , const CAP : usize > Sync for Drain < 'a , T , CAP > { }
    };
}

impl_59!()