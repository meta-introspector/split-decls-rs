macro_rules! deps {
    () => {
        PropertyParserBorrowed!();
    };
}

macro_rules! PropertyParser {
    () => {
        deps!();
        # [doc = " A struct capable of looking up a property value from a string name."] # [doc = " Access its data by calling [`Self::as_borrowed()`] and using the methods on"] # [doc = " [`PropertyParserBorrowed`]."] # [doc = ""] # [doc = " The name can be a short name (`Lu`), a long name(`Uppercase_Letter`),"] # [doc = " or an alias."] # [doc = ""] # [doc = " Property names can be looked up using \"strict\" matching (looking for a name"] # [doc = " that matches exactly), or \"loose matching\", where the name is allowed to deviate"] # [doc = " in terms of ASCII casing, whitespace, underscores, and hyphens."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::props::GeneralCategory;"] # [doc = " use icu::properties::PropertyParser;"] # [doc = ""] # [doc = " let lookup = PropertyParser::<GeneralCategory>::new();"] # [doc = " // short name for value"] # [doc = " assert_eq!("] # [doc = "     lookup.get_strict(\"Lu\"),"] # [doc = "     Some(GeneralCategory::UppercaseLetter)"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     lookup.get_strict(\"Pd\"),"] # [doc = "     Some(GeneralCategory::DashPunctuation)"] # [doc = " );"] # [doc = " // long name for value"] # [doc = " assert_eq!("] # [doc = "     lookup.get_strict(\"Uppercase_Letter\"),"] # [doc = "     Some(GeneralCategory::UppercaseLetter)"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     lookup.get_strict(\"Dash_Punctuation\"),"] # [doc = "     Some(GeneralCategory::DashPunctuation)"] # [doc = " );"] # [doc = " // name has incorrect casing"] # [doc = " assert_eq!(lookup.get_strict(\"dashpunctuation\"), None);"] # [doc = " // loose matching of name"] # [doc = " assert_eq!("] # [doc = "     lookup.get_loose(\"dash-punctuation\"),"] # [doc = "     Some(GeneralCategory::DashPunctuation)"] # [doc = " );"] # [doc = " // fake property"] # [doc = " assert_eq!(lookup.get_strict(\"Animated_Gif\"), None);"] # [doc = " ```"] # [derive (Debug)] pub struct PropertyParser < T > { map : DataPayload < ErasedMarker < PropertyValueNameToEnumMap < 'static > > > , markers : PhantomData < fn () -> T > , }
    };
}

PropertyParser!()