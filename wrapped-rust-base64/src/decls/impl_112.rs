macro_rules! deps {
    () => {
        GeneralPurposeConfig!();
        Config!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl Config for GeneralPurposeConfig { fn encode_padding (& self) -> bool { self . encode_padding } }
    };
}

impl_112!()