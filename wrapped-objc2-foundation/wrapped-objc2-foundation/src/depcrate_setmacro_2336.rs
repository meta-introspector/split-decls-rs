// Generated macro for macro_2336 (macro)
macro_rules! Depcrate_setmacro_2336 {
() => {
// Module: crate::set
// Provides: {"macro_2336"}
// Dependencies: {}
# [cfg (feature = "NSEnumerator")] __impl_into_iter ! { impl < ObjectType : Message > IntoIterator for & NSSet < ObjectType > { type IntoIter = Iter <'_ , ObjectType >; } impl < ObjectType : Message > IntoIterator for & NSMutableSet < ObjectType > { type IntoIter = Iter <'_ , ObjectType >; } impl < ObjectType : Message > IntoIterator for Retained < NSSet < ObjectType >> { # [uses (new)] type IntoIter = IntoIter < ObjectType >; } impl < ObjectType : Message > IntoIterator for Retained < NSMutableSet < ObjectType >> { # [uses (new_mutable)] type IntoIter = IntoIter < ObjectType >; } }
};
}
