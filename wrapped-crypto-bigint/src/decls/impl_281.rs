macro_rules! deps {
    () => {
        RandomBitsError!();
    };
}

macro_rules! impl_281 {
    () => {
        deps!();
        # [cfg (feature = "rand_core")] impl < T > core :: error :: Error for RandomBitsError < T > where T : Debug + fmt :: Display { }
    };
}

impl_281!()