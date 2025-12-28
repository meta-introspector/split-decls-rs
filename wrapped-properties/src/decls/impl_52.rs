macro_rules! deps {
    () => {
        Script!();
        PropertyNamesShortBorrowed!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl PropertyNamesShortBorrowed < '_ , Script > { # [doc = " Gets the \"name\" of a script property as a `icu::locale::subtags::Script`."] # [doc = ""] # [doc = " This method is available only on `PropertyNamesShortBorrowed<Script>`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use icu::locale::subtags::script;"] # [doc = " use icu::properties::props::Script;"] # [doc = " use icu::properties::PropertyNamesShort;"] # [doc = ""] # [doc = " let lookup = PropertyNamesShort::<Script>::new();"] # [doc = " assert_eq!("] # [doc = "     lookup.get_locale_script(Script::Brahmi),"] # [doc = "     Some(script!(\"Brah\"))"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     lookup.get_locale_script(Script::Hangul),"] # [doc = "     Some(script!(\"Hang\"))"] # [doc = " );"] # [doc = " ```"] # [doc = ""] # [doc = " For the reverse direction, use property parsing as normal:"] # [doc = " ```"] # [doc = " use icu::locale::subtags::script;"] # [doc = " use icu::properties::props::Script;"] # [doc = " use icu::properties::PropertyParser;"] # [doc = ""] # [doc = " let parser = PropertyParser::<Script>::new();"] # [doc = " assert_eq!("] # [doc = "     parser.get_strict(script!(\"Brah\").as_str()),"] # [doc = "     Some(Script::Brahmi)"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     parser.get_strict(script!(\"Hang\").as_str()),"] # [doc = "     Some(Script::Hangul)"] # [doc = " );"] # [doc = " ```"] # [inline] pub fn get_locale_script (self , property : Script) -> Option < icu_locale_core :: subtags :: Script > { let prop = usize :: try_from (property . to_u32 ()) . ok () ? ; self . map . map . get (prop) . and_then (| o | o . 0) } }
    };
}

impl_52!();