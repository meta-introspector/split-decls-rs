macro_rules! deps {
    () => {
        TrySendError!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < T : core :: any :: Any > std :: error :: Error for TrySendError < T > { }
    };
}

impl_50!();