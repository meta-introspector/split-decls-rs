macro_rules! deps {
    () => {
        Sample!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl < A > ops :: Deref for Sample < A > { type Target = [A] ; fn deref (& self) -> & [A] { & self . 0 } }
    };
}

impl_339!()