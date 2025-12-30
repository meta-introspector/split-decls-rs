// Generated macro for IndicSyllabicCategory (struct)
macro_rules! Depcrate_propsIndicSyllabicCategory {
() => {
// Module: crate::props
// Provides: {"IndicSyllabicCategory"}
// Dependencies: {}
# [doc = " Property Indic_Syllabic_Category."] # [doc = " See UAX #44:"] # [doc = " <https://www.unicode.org/reports/tr44/#Indic_Syllabic_Category>."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::{props::IndicSyllabicCategory, CodePointMapData};"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<IndicSyllabicCategory>::new().get('a'),"] # [doc = "     IndicSyllabicCategory::Other"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<IndicSyllabicCategory>::new().get('\\u{0900}'),"] # [doc = "     IndicSyllabicCategory::Bindu"] # [doc = " ); // U+0900: DEVANAGARI SIGN INVERTED CANDRABINDU"] # [doc = " ```"] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [allow (clippy :: exhaustive_structs)] # [repr (transparent)] pub struct IndicSyllabicCategory (pub (crate) u8) ;
};
}
