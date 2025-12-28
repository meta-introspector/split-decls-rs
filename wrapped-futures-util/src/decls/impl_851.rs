macro_rules! deps {
    () => {
        Send!();
        IterPinMut!();
    };
}

macro_rules! impl_851 {
    () => {
        deps!();
        unsafe impl < Fut : Send > Send for IterPinMut < '_ , Fut > { }
    };
}

impl_851!()