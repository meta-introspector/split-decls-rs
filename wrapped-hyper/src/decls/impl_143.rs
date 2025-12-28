macro_rules! deps {
    () => {
        OnInformational!();
        Response!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl OnInformational { pub (crate) fn call (& self , res : http :: Response < () >) { self . 0 . on_informational (res) ; } }
    };
}

impl_143!();