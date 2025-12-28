macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < T > Clone for Pending < T > { fn clone (& self) -> Self { pending () } }
    };
}

impl_157!()