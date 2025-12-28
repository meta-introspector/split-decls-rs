macro_rules! deps {
    () => {
        MapKey!();
        Read!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < 'de , 'a , R > MapKey < 'a , R > where R : Read < 'de > , { deserialize_numeric_key ! (deserialize_number , deserialize_number) ; }
    };
}

impl_44!();