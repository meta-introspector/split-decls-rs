// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl < T : Clone + PartialOrd + PartialEq > AnyRange < T > { # [doc = " Returns the bounds, useful for slicing."] # [doc = " # Example"] # [doc = " ```"] # [doc = " use any_range::AnyRange;"] # [doc = " let r = AnyRange::from(1..3usize);"] # [doc = " assert_eq!(\"bc\", &\"abcd\"[r.bounds()]);"] # [doc = " ```"] # [must_use] pub fn bounds (& self) -> (Bound < T > , Bound < T >) { (self . start_bound () . cloned () , self . end_bound () . cloned ()) } # [doc = " Returns true if item is contained in the range."] pub fn contains (& self , value : & T) -> bool { RangeBounds :: contains (self , value) } # [doc = " Returns the start value as a Bound."] pub fn start_bound (& self) -> Bound < & T > { RangeBounds :: start_bound (self) } # [doc = " Returns the end value as a Bound."] pub fn end_bound (& self) -> Bound < & T > { RangeBounds :: end_bound (self) } fn order (& self) -> u8 { match self { Self :: Range (_) => 0 , Self :: RangeFrom (_) => 1 , Self :: RangeFull (_) => 2 , Self :: RangeInclusive (_) => 3 , Self :: RangeTo (_) => 4 , Self :: RangeToInclusive (_) => 5 , } } }
};
}
