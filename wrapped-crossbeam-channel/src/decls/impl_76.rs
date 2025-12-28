macro_rules! deps {
    () => {
        TrySendError!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < T : Send > error :: Error for TrySendError < T > { }
    };
}

impl_76!();