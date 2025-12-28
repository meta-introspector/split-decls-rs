macro_rules! deps {
    () => {
        CodePointMapData!();
    };
}

macro_rules! LineBreak {
    () => {
        deps!();
        # [doc = " Enumerated property Line_Break."] # [doc = ""] # [doc = " See \"Line Breaking Properties\" in UAX #14 for the summary of each property"] # [doc = " value: <https://www.unicode.org/reports/tr14/#Properties>"] # [doc = ""] # [doc = " The numeric value is compatible with `ULineBreak` in ICU4C."] # [doc = ""] # [doc = " **Note:** Use `icu::segmenter` for an all-in-one break iterator implementation."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::{props::LineBreak, CodePointMapData};"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<LineBreak>::new().get(')'),"] # [doc = "     LineBreak::CloseParenthesis"] # [doc = " ); // U+0029: Right Parenthesis"] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<LineBreak>::new().get('ぁ'),"] # [doc = "     LineBreak::ConditionalJapaneseStarter"] # [doc = " ); //U+3041: Hiragana Letter Small A"] # [doc = " ```"] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [allow (clippy :: exhaustive_structs)] # [repr (transparent)] pub struct LineBreak (pub (crate) u8) ;
    };
}

LineBreak!()