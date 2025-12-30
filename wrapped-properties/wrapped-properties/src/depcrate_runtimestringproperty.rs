// Generated macro for StringProperty (enum)
macro_rules! Depcrate_runtimeStringProperty {
() => {
// Module: crate::runtime
// Provides: {"StringProperty"}
// Dependencies: {}
# [doc = " This type can represent any Unicode string property."] # [doc = ""] # [doc = " This is intended to be used in situations where the exact unicode property needed is"] # [doc = " only known at runtime, for example in regex engines."] # [doc = ""] # [doc = " The values are intended to be identical to ICU4C's UProperty enum"] # [non_exhaustive] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] # [allow (dead_code)] # [allow (missing_docs)] enum StringProperty { Age = 0x4000 , BidiMirroringGlyph = 0x4001 , BidiPairedBracket = 0x400D , CaseFolding = 0x4002 , ISOComment = 0x4003 , LowercaseMapping = 0x4004 , Name = 0x4005 , SimpleCaseFolding = 0x4006 , SimpleLowercaseMapping = 0x4007 , SimpleTitlecaseMapping = 0x4008 , SimpleUppercaseMapping = 0x4009 , TitlecaseMapping = 0x400A , Unicode1Name = 0x400B , UppercaseMapping = 0x400C , }
};
}
