macro_rules! deps {
    () => {
        AssertAsync!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl < T > Unpin for AssertAsync < T > { }
    };
}

impl_193!()