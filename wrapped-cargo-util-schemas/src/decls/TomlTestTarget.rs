macro_rules! deps {
    () => {
        TomlTarget!();
    };
}

macro_rules! TomlTestTarget {
    () => {
        deps!();
        pub type TomlTestTarget = TomlTarget ;
    };
}

TomlTestTarget!()