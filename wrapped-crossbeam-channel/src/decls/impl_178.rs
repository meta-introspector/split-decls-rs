macro_rules! deps {
    () => {
        Select!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        unsafe impl Sync for Select < '_ > { }
    };
}

impl_178!()