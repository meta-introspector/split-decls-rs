macro_rules! deps {
    () => {
        CodePointMapData!();
    };
}

macro_rules! SentenceBreak {
    () => {
        deps!();
        # [doc = " Enumerated property Sentence_Break."] # [doc = ""] # [doc = " See \"Default Sentence Boundary Specification\" in UAX #29 for the summary of"] # [doc = " each property value:"] # [doc = " <https://www.unicode.org/reports/tr29/#Default_Word_Boundaries>."] # [doc = ""] # [doc = " **Note:** Use `icu::segmenter` for an all-in-one break iterator implementation."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::{props::SentenceBreak, CodePointMapData};"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<SentenceBreak>::new().get('９'),"] # [doc = "     SentenceBreak::Numeric"] # [doc = " ); // U+FF19: Fullwidth Digit Nine"] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<SentenceBreak>::new().get(','),"] # [doc = "     SentenceBreak::SContinue"] # [doc = " ); // U+002C: Comma"] # [doc = " ```"] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [allow (clippy :: exhaustive_structs)] # [repr (transparent)] pub struct SentenceBreak (pub (crate) u8) ;
    };
}

SentenceBreak!();