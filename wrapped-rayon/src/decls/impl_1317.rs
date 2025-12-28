macro_rules! deps {
    () => {
        SplitTerminator!();
    };
}

macro_rules! impl_1317 {
    () => {
        deps!();
        impl < 'ch , P : Pattern > SplitTerminator < 'ch , P > { fn new (chars : & 'ch str , terminator : P) -> Self { SplitTerminator { chars , terminator } } }
    };
}

impl_1317!()