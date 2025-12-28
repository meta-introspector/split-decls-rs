macro_rules! deps {
    () => {
        Config!();
        NaiveConfig!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl Config for NaiveConfig { fn encode_padding (& self) -> bool { self . encode_padding } }
    };
}

impl_133!();