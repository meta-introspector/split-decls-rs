macro_rules! deps {
    () => {
        SignatureRef!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < 'a > Sequence < 'a > for SignatureRef < 'a > { }
    };
}

impl_38!()