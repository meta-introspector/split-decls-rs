// Generated macro for GraphemeClusterBreak (struct)
macro_rules! Depcrate_propsGraphemeClusterBreak {
() => {
// Module: crate::props
// Provides: {"GraphemeClusterBreak"}
// Dependencies: {}
# [doc = " Enumerated property Grapheme_Cluster_Break."] # [doc = ""] # [doc = " See \"Default Grapheme Cluster Boundary Specification\" in UAX #29 for the"] # [doc = " summary of each property value:"] # [doc = " <https://www.unicode.org/reports/tr29/#Default_Grapheme_Cluster_Table>"] # [doc = ""] # [doc = " **Note:** Use `icu::segmenter` for an all-in-one break iterator implementation."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::{props::GraphemeClusterBreak, CodePointMapData};"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<GraphemeClusterBreak>::new().get('🇦'),"] # [doc = "     GraphemeClusterBreak::RegionalIndicator"] # [doc = " ); // U+1F1E6: Regional Indicator Symbol Letter A"] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<GraphemeClusterBreak>::new().get('ำ'),"] # [doc = "     GraphemeClusterBreak::SpacingMark"] # [doc = " ); //U+0E33: Thai Character Sara Am"] # [doc = " ```"] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [allow (clippy :: exhaustive_structs)] # [repr (transparent)] pub struct GraphemeClusterBreak (pub (crate) u8) ;
};
}
