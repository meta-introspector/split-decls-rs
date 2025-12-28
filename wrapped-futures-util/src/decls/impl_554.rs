macro_rules! deps {
    () => {
        SplitStream!();
    };
}

macro_rules! impl_554 {
    () => {
        deps!();
        impl < S > Unpin for SplitStream < S > { }
    };
}

impl_554!()