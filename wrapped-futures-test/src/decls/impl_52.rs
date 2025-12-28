macro_rules! deps {
    () => {
        StreamTestExt!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < St > StreamTestExt for St where St : Stream { }
    };
}

impl_52!()