macro_rules! deps {
    () => {
        Extension!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < T > Extension for T where T : std :: fmt :: Debug + Clone + std :: any :: Any + Send + Sync + 'static { }
    };
}

impl_102!()