macro_rules! deps {
    () => {
        SizedTypeProperties!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < T > SizedTypeProperties for T { }
    };
}

impl_36!()