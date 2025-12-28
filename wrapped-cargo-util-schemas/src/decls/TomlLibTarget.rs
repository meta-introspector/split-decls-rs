macro_rules! deps {
    () => {
        TomlTarget!();
    };
}

macro_rules! TomlLibTarget {
    () => {
        deps!();
        pub type TomlLibTarget = TomlTarget ;
    };
}

TomlLibTarget!();