macro_rules! deps {
    () => {
        PropertyNamesLongBorrowed!();
        NamedEnumeratedProperty!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl < 'a , T : NamedEnumeratedProperty > PropertyNamesLongBorrowed < 'a , T > { # [doc = " Get the property name given a value"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use icu::properties::props::CanonicalCombiningClass;"] # [doc = " use icu::properties::PropertyNamesLong;"] # [doc = ""] # [doc = " let lookup = PropertyNamesLong::<CanonicalCombiningClass>::new();"] # [doc = " assert_eq!("] # [doc = "     lookup.get(CanonicalCombiningClass::KanaVoicing),"] # [doc = "     Some(\"Kana_Voicing\")"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     lookup.get(CanonicalCombiningClass::AboveLeft),"] # [doc = "     Some(\"Above_Left\")"] # [doc = " );"] # [doc = " ```"] # [inline] pub fn get (self , property : T) -> Option < & 'a str > { self . map . get (property . to_u32 ()) } }
    };
}

impl_241!()