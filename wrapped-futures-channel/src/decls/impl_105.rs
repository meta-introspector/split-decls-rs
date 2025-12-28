macro_rules! deps {
    () => {
        Sender!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < T > Unpin for Sender < T > { }
    };
}

impl_105!()