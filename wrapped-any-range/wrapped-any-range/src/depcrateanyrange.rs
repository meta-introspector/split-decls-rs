// Generated macro for AnyRange (enum)
macro_rules! DepcrateAnyRange {
() => {
// Module: crate
// Provides: {"AnyRange"}
// Dependencies: {}
# [doc = " An enum that can hold any Range* type."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " use any_range::AnyRange;"] # [doc = " let r = AnyRange::from(3..5);"] # [doc = " assert!(r.contains(&3));"] # [doc = " assert_eq!(\"de\", &\"abcdefg\"[r.bounds()]);"] # [doc = " ```"] # [derive (Clone , PartialEq , Eq , Hash)] pub enum AnyRange < T : Clone + PartialOrd + PartialEq > { Range (Range < T >) , RangeFrom (RangeFrom < T >) , RangeFull (RangeFull) , RangeInclusive (RangeInclusive < T >) , RangeTo (RangeTo < T >) , RangeToInclusive (RangeToInclusive < T >) , }
};
}
