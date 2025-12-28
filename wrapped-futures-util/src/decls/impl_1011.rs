macro_rules! deps {
    () => {
        Stream01CompatExt!();
    };
}

macro_rules! impl_1011 {
    () => {
        deps!();
        impl < St : Stream01 > Stream01CompatExt for St { }
    };
}

impl_1011!();