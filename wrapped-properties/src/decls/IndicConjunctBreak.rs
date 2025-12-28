macro_rules! deps {
    () => {
        CodePointMapData!();
    };
}

macro_rules! IndicConjunctBreak {
    () => {
        deps!();
        # [doc = " Property Indic_Conjunct_Break."] # [doc = " See UAX #44:"] # [doc = " <https://www.unicode.org/reports/tr44/#Indic_Conjunct_Break>."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::{props::IndicConjunctBreak, CodePointMapData};"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<IndicConjunctBreak>::new().get('a'),"] # [doc = "     IndicConjunctBreak::None"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<IndicConjunctBreak>::new().get('\\u{094d}'),"] # [doc = "     IndicConjunctBreak::Linker"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<IndicConjunctBreak>::new().get('\\u{0915}'),"] # [doc = "     IndicConjunctBreak::Consonant"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<IndicConjunctBreak>::new().get('\\u{0300}'),"] # [doc = "     IndicConjunctBreak::Extend"] # [doc = " );"] # [doc = " ```"] # [doc (hidden)] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [allow (clippy :: exhaustive_structs)] # [repr (transparent)] pub struct IndicConjunctBreak (pub (crate) u8) ;
    };
}

IndicConjunctBreak!()