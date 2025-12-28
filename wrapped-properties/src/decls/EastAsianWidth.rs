macro_rules! deps {
    () => {
        CodePointMapData!();
    };
}

macro_rules! EastAsianWidth {
    () => {
        deps!();
        # [doc = " Enumerated property East_Asian_Width."] # [doc = ""] # [doc = " See \"Definition\" in UAX #11 for the summary of each property value:"] # [doc = " <https://www.unicode.org/reports/tr11/#Definitions>"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::{props::EastAsianWidth, CodePointMapData};"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<EastAsianWidth>::new().get('ｱ'),"] # [doc = "     EastAsianWidth::Halfwidth"] # [doc = " ); // U+FF71: Halfwidth Katakana Letter A"] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<EastAsianWidth>::new().get('ア'),"] # [doc = "     EastAsianWidth::Wide"] # [doc = " ); //U+30A2: Katakana Letter A"] # [doc = " ```"] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [allow (clippy :: exhaustive_structs)] # [repr (transparent)] pub struct EastAsianWidth (pub (crate) u8) ;
    };
}

EastAsianWidth!();