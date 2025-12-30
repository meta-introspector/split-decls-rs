// Generated macro for macro_90 (macro)
macro_rules! Depcrate_arraymacro_90 {
() => {
// Module: crate::array
// Provides: {"macro_90"}
// Dependencies: {}
# [cfg (feature = "NSEnumerator")] __impl_into_iter ! { impl < ObjectType : Message > IntoIterator for & NSArray < ObjectType > { type IntoIter = Iter <'_ , ObjectType >; } impl < ObjectType : Message > IntoIterator for & NSMutableArray < ObjectType > { type IntoIter = Iter <'_ , ObjectType >; } impl < ObjectType : Message > IntoIterator for Retained < NSArray < ObjectType >> { # [uses (new)] type IntoIter = IntoIter < ObjectType >; } impl < ObjectType : Message > IntoIterator for Retained < NSMutableArray < ObjectType >> { # [uses (new_mutable)] type IntoIter = IntoIter < ObjectType >; } }
};
}
