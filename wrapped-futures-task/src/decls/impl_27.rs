macro_rules! deps {
    () => {
        WakerRef!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl Deref for WakerRef < '_ > { type Target = Waker ; # [inline] fn deref (& self) -> & Waker { & self . waker } }
    };
}

impl_27!()