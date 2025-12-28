macro_rules! deps {
    () => {
        RawValue!();
    };
}

macro_rules! impl_607 {
    () => {
        deps!();
        impl Clone for Box < RawValue > { fn clone (& self) -> Self { (* * self) . to_owned () } }
    };
}

impl_607!()