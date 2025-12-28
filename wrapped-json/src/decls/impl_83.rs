macro_rules! deps {
    () => {
        Value!();
        Map!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl Eq for Map < String , Value > { }
    };
}

impl_83!()