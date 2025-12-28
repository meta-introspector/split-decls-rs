macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl < 'a , X , Y > Copy for Data < 'a , X , Y > { }
    };
}

impl_304!();