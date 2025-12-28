macro_rules! deps {
    () => {
        CodePointMapData!();
    };
}

macro_rules! BidiClass {
    () => {
        deps!();
        # [doc = " Enumerated property Bidi_Class"] # [doc = ""] # [doc = " These are the categories required by the Unicode Bidirectional Algorithm."] # [doc = " For the property values, see [Bidirectional Class Values](https://unicode.org/reports/tr44/#Bidi_Class_Values)."] # [doc = " For more information, see [Unicode Standard Annex #9](https://unicode.org/reports/tr41/tr41-28.html#UAX9)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::{props::BidiClass, CodePointMapData};"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<BidiClass>::new().get('y'),"] # [doc = "     BidiClass::LeftToRight"] # [doc = " ); // U+0079"] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<BidiClass>::new().get('ع'),"] # [doc = "     BidiClass::ArabicLetter"] # [doc = " ); // U+0639"] # [doc = " ```"] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [allow (clippy :: exhaustive_structs)] # [repr (transparent)] pub struct BidiClass (pub (crate) u8) ;
    };
}

BidiClass!()