macro_rules! deps {
    () => {
        FromStream!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < S : Stream > FromStream < S > { pub (crate) fn new (stream : S) -> Self { Self { stream } } }
    };
}

impl_156!();