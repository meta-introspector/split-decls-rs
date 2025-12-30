// Generated macro for impl_864 (impl)
macro_rules! Depcrate_validation_rules_overlapping_fields_can_be_mergedimpl_864 {
() => {
// Module: crate::validation::rules::overlapping_fields_can_be_merged
// Provides: {"impl_864"}
// Dependencies: {}
impl < 'a > PairSet < 'a > { fn new () -> PairSet < 'a > { PairSet { data : HashMap :: new () , } } fn contains (& self , a : & 'a str , b : & 'a str , mutex : bool) -> bool { if let Some (result) = self . data . get (a) . and_then (| s | s . get (b)) { if ! mutex { ! result } else { true } } else { false } } fn insert (& mut self , a : & 'a str , b : & 'a str , mutex : bool) { self . data . entry (a) . or_default () . insert (b , mutex) ; self . data . entry (b) . or_default () . insert (a , mutex) ; } }
};
}
