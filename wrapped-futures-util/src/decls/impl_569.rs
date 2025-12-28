macro_rules! deps {
    () => {
        ReuniteError!();
    };
}

macro_rules! impl_569 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < T : core :: any :: Any , Item > std :: error :: Error for ReuniteError < T , Item > { }
    };
}

impl_569!();