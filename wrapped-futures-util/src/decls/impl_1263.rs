macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! impl_1263 {
    () => {
        deps!();
        impl < T : Unpin > Inner < T > { unsafe fn into_value (mut self) -> T { self . value . take () . unwrap () . into_inner () } }
    };
}

impl_1263!()