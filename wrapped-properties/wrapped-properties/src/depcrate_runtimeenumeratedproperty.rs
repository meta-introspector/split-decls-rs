// Generated macro for EnumeratedProperty (enum)
macro_rules! Depcrate_runtimeEnumeratedProperty {
() => {
// Module: crate::runtime
// Provides: {"EnumeratedProperty"}
// Dependencies: {}
# [doc = " This type can represent any enumerated Unicode property."] # [doc = ""] # [doc = " This is intended to be used in situations where the exact unicode property needed is"] # [doc = " only known at runtime, for example in regex engines."] # [doc = ""] # [doc = " The values are intended to be identical to ICU4C's UProperty enum"] # [non_exhaustive] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] # [allow (dead_code)] # [allow (missing_docs)] enum EnumeratedProperty { BidiClass = 0x1000 , BidiPairedBracketType = 0x1015 , Block = 0x1001 , CombiningClass = 0x1002 , DecompositionType = 0x1003 , EastAsianWidth = 0x1004 , GeneralCategory = 0x1005 , GraphemeClusterBreak = 0x1012 , HangulSyllableType = 0x100B , IndicConjunctBreak = 0x101A , IndicPositionalCategory = 0x1016 , IndicSyllabicCategory = 0x1017 , JoiningGroup = 0x1006 , JoiningType = 0x1007 , LeadCanonicalCombiningClass = 0x1010 , LineBreak = 0x1008 , NFCQuickCheck = 0x100E , NFDQuickCheck = 0x100C , NFKCQuickCheck = 0x100F , NFKDQuickCheck = 0x100D , NumericType = 0x1009 , Script = 0x100A , SentenceBreak = 0x1013 , TrailCanonicalCombiningClass = 0x1011 , VerticalOrientation = 0x1018 , WordBreak = 0x1014 , }
};
}
