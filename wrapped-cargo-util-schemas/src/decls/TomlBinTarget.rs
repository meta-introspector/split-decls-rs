macro_rules! deps {
    () => {
        TomlTarget!();
    };
}

macro_rules! TomlBinTarget {
    () => {
        deps!();
        pub type TomlBinTarget = TomlTarget ;
    };
}

TomlBinTarget!()