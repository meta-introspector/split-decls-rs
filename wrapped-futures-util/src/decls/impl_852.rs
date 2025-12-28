macro_rules! deps {
    () => {
        IterPinMut!();
    };
}

macro_rules! impl_852 {
    () => {
        deps!();
        unsafe impl < Fut : Sync > Sync for IterPinMut < '_ , Fut > { }
    };
}

impl_852!();