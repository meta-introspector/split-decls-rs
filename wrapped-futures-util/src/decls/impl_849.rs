macro_rules! deps {
    () => {
        IterPinRef!();
        Send!();
    };
}

macro_rules! impl_849 {
    () => {
        deps!();
        unsafe impl < Fut : Send > Send for IterPinRef < '_ , Fut > { }
    };
}

impl_849!();