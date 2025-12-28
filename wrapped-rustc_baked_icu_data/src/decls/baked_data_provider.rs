macro_rules! deps {
    () => {
        BakedDataProvider!();
    };
}

macro_rules! baked_data_provider {
    () => {
        deps!();
        pub const fn baked_data_provider () -> BakedDataProvider { BakedDataProvider }
    };
}

baked_data_provider!()