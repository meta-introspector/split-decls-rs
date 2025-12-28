macro_rules! deps {
    () => {
        CodePointMapData!();
    };
}

macro_rules! CanonicalCombiningClass {
    () => {
        deps!();
        # [doc = " Property Canonical_Combining_Class."] # [doc = " See UAX #15:"] # [doc = " <https://www.unicode.org/reports/tr15/>."] # [doc = ""] # [doc = " See `icu::normalizer::properties::CanonicalCombiningClassMap` for the API"] # [doc = " to look up the Canonical_Combining_Class property by scalar value."] # [doc = ""] # [doc = " **Note:** See `icu::normalizer::CanonicalCombiningClassMap` for the preferred API"] # [doc = " to look up the Canonical_Combining_Class property by scalar value."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::{props::CanonicalCombiningClass, CodePointMapData};"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<CanonicalCombiningClass>::new().get('a'),"] # [doc = "     CanonicalCombiningClass::NotReordered"] # [doc = " ); // U+0061: LATIN SMALL LETTER A"] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<CanonicalCombiningClass>::new().get('\\u{0301}'),"] # [doc = "     CanonicalCombiningClass::Above"] # [doc = " ); // U+0301: COMBINING ACUTE ACCENT"] # [doc = " ```"] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [allow (clippy :: exhaustive_structs)] # [repr (transparent)] pub struct CanonicalCombiningClass (pub (crate) u8) ;
    };
}

CanonicalCombiningClass!()