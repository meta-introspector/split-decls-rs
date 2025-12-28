macro_rules! deps {
    () => {
        ScriptWithExtensionsProperty!();
    };
}

macro_rules! macro_363 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyScriptWithExtensionsV1`"] PropertyScriptWithExtensionsV1 , ScriptWithExtensionsProperty <'static >, is_singleton = true) ;
    };
}

macro_363!()