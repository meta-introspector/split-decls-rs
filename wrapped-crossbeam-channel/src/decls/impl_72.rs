macro_rules! deps {
    () => {
        SendError!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < T : Send > error :: Error for SendError < T > { }
    };
}

impl_72!()