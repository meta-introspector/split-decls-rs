macro_rules! deps {
    () => {
        Split!();
    };
}

macro_rules! impl_1309 {
    () => {
        deps!();
        impl < 'ch , P : Pattern > Split < 'ch , P > { fn new (chars : & 'ch str , separator : P) -> Self { Split { chars , separator } } }
    };
}

impl_1309!()