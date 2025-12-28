macro_rules! deps {
    () => {
        TomlLint!();
    };
}

macro_rules! TomlToolLints {
    () => {
        deps!();
        pub type TomlToolLints = BTreeMap < String , TomlLint > ;
    };
}

TomlToolLints!()