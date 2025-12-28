macro_rules! deps {
    () => {
        TomlToolLints!();
    };
}

macro_rules! TomlLints {
    () => {
        deps!();
        pub type TomlLints = BTreeMap < String , TomlToolLints > ;
    };
}

TomlLints!();