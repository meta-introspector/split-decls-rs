// Generated macro for VariableValue (enum)
macro_rules! Depcrate_unicodeset_parse_parseVariableValue {
() => {
// Module: crate::unicodeset_parse::parse
// Provides: {"VariableValue"}
// Dependencies: {}
# [doc = " The value of a variable in a UnicodeSet. Used as value type in [`VariableMap`]."] # [derive (Debug , Clone)] # [non_exhaustive] pub enum VariableValue < 'a > { # [doc = " A UnicodeSet, represented as a [`CodePointInversionListAndStringList`](CodePointInversionListAndStringList)."] UnicodeSet (CodePointInversionListAndStringList < 'a >) , # [doc = " A single code point."] Char (char) , # [doc = " A string. It is guaranteed that when returned from a VariableMap, this variant contains never exactly one code point."] String (Cow < 'a , str >) , }
};
}
