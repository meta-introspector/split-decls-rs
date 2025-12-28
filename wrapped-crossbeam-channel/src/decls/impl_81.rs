macro_rules! deps {
    () => {
        SendTimeoutError!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < T : Send > error :: Error for SendTimeoutError < T > { }
    };
}

impl_81!()