macro_rules! deps {
    () => {
        CodePointMapData!();
    };
}

macro_rules! WordBreak {
    () => {
        deps!();
        # [doc = " Enumerated property Word_Break."] # [doc = ""] # [doc = " See \"Default Word Boundary Specification\" in UAX #29 for the summary of"] # [doc = " each property value:"] # [doc = " <https://www.unicode.org/reports/tr29/#Default_Word_Boundaries>."] # [doc = ""] # [doc = " **Note:** Use `icu::segmenter` for an all-in-one break iterator implementation."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::{props::WordBreak, CodePointMapData};"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<WordBreak>::new().get('.'),"] # [doc = "     WordBreak::MidNumLet"] # [doc = " ); // U+002E: Full Stop"] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<WordBreak>::new().get('，'),"] # [doc = "     WordBreak::MidNum"] # [doc = " ); // U+FF0C: Fullwidth Comma"] # [doc = " ```"] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [allow (clippy :: exhaustive_structs)] # [repr (transparent)] pub struct WordBreak (pub (crate) u8) ;
    };
}

WordBreak!()