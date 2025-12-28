macro_rules! deps {
    () => {
        ReuniteError!();
    };
}

macro_rules! impl_1214 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < T : core :: any :: Any > std :: error :: Error for ReuniteError < T > { }
    };
}

impl_1214!()