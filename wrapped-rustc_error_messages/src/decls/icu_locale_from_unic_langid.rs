macro_rules! icu_locale_from_unic_langid {
    () => {
        fn icu_locale_from_unic_langid (lang : LanguageIdentifier) -> Option < icu_locale :: Locale > { icu_locale :: Locale :: try_from_str (& lang . to_string ()) . ok () }
    };
}

icu_locale_from_unic_langid!();