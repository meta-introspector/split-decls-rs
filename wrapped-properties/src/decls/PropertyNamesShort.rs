macro_rules! deps {
    () => {
        NamedEnumeratedProperty!();
        PropertyNamesShortBorrowed!();
    };
}

macro_rules! PropertyNamesShort {
    () => {
        deps!();
        # [doc = " A struct capable of looking up a property name from a value"] # [doc = " Access its data by calling [`Self::as_borrowed()`] and using the methods on"] # [doc = " [`PropertyNamesShortBorrowed`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::props::CanonicalCombiningClass;"] # [doc = " use icu::properties::PropertyNamesShort;"] # [doc = ""] # [doc = " let names = PropertyNamesShort::<CanonicalCombiningClass>::new();"] # [doc = " assert_eq!(names.get(CanonicalCombiningClass::KanaVoicing), Some(\"KV\"));"] # [doc = " assert_eq!(names.get(CanonicalCombiningClass::AboveLeft), Some(\"AL\"));"] # [doc = " ```"] pub struct PropertyNamesShort < T : NamedEnumeratedProperty > { map : DataPayload < ErasedMarker < T :: DataStructShort > > , }
    };
}

PropertyNamesShort!()