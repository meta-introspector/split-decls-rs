macro_rules! default_translator {
    () => {
        pub fn default_translator () -> Translator { Translator :: with_fallback_bundle (DEFAULT_LOCALE_RESOURCES . to_vec () , false) }
    };
}

default_translator!()