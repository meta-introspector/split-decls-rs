macro_rules! deps {
    () => {
        Scope!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        unsafe impl Sync for Scope < '_ > { }
    };
}

impl_156!()