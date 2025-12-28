macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl < T > Unpin for Ready < T > { }
    };
}

impl_193!()