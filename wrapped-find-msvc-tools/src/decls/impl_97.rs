macro_rules! deps {
    () => {
        Repr!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        unsafe impl Sync for Repr { }
    };
}

impl_97!();