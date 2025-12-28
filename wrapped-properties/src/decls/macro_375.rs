macro_rules! deps {
    () => {
        ScriptWithExtensionsProperty!();
    };
}

macro_rules! macro_375 {
    () => {
        deps!();
        icu_provider :: data_struct ! (ScriptWithExtensionsProperty <'_ >, # [cfg (feature = "datagen")]) ;
    };
}

macro_375!();