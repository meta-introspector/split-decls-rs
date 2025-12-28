macro_rules! deps {
    () => {
        TomlTarget!();
    };
}

macro_rules! TomlExampleTarget {
    () => {
        deps!();
        pub type TomlExampleTarget = TomlTarget ;
    };
}

TomlExampleTarget!()