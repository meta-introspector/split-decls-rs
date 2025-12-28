macro_rules! deps {
    () => {
        FluentBundle!();
    };
}

macro_rules! new_bundle {
    () => {
        deps!();
        fn new_bundle (locales : Vec < LanguageIdentifier >) -> FluentBundle { IntoDynSyncSend (fluent_bundle :: bundle :: FluentBundle :: new_concurrent (locales)) }
    };
}

new_bundle!()