macro_rules! deps {
    () => {
        TomlTarget!();
    };
}

macro_rules! TomlBenchTarget {
    () => {
        deps!();
        pub type TomlBenchTarget = TomlTarget ;
    };
}

TomlBenchTarget!();