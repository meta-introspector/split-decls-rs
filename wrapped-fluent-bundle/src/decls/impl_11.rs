macro_rules! deps {
    () => {
        FluentBundle!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < R > FluentBundle < R , IntlLangMemoizer > { # [doc = " Constructs a `FluentBundle`. The first element in `locales` should be the"] # [doc = " language this bundle represents, and will be used to determine the"] # [doc = " correct plural rules for this bundle. You can optionally provide extra"] # [doc = " languages in the list; they will be used as fallback date and time"] # [doc = " formatters if a formatter for the primary language is unavailable."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use fluent_bundle::FluentBundle;"] # [doc = " use fluent_bundle::FluentResource;"] # [doc = " use unic_langid::langid;"] # [doc = ""] # [doc = " let langid_en = langid!(\"en-US\");"] # [doc = " let mut bundle: FluentBundle<FluentResource> = FluentBundle::new(vec![langid_en]);"] # [doc = " ```"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This will panic if no formatters can be found for the locales."] pub fn new (locales : Vec < LanguageIdentifier >) -> Self { let first_locale = locales . first () . cloned () . unwrap_or_default () ; Self { locales , resources : vec ! [] , entries : FxHashMap :: default () , intls : IntlLangMemoizer :: new (first_locale) , use_isolating : true , transform : None , formatter : None , } } }
    };
}

impl_11!()