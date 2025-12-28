macro_rules! deps {
    () => {
        ReuniteError!();
    };
}

macro_rules! impl_1268 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < T : core :: any :: Any > std :: error :: Error for ReuniteError < T > { }
    };
}

impl_1268!();