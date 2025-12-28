macro_rules! deps {
    () => {
        KeyValue!();
    };
}

macro_rules! TomlSection {
    () => {
        deps!();
        pub struct TomlSection { pub items : Punctuated < KeyValue , Token ! [,] > , }
    };
}

TomlSection!()