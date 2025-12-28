macro_rules! deps {
    () => {
        IterPinRef!();
    };
}

macro_rules! impl_850 {
    () => {
        deps!();
        unsafe impl < Fut : Sync > Sync for IterPinRef < '_ , Fut > { }
    };
}

impl_850!()