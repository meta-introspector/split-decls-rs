// Generated macro for macro_220 (macro)
macro_rules! Depcrate_enumeratormacro_220 {
() => {
// Module: crate::enumerator
// Provides: {"macro_220"}
// Dependencies: {}
__impl_into_iter ! { impl < ObjectType : Message > IntoIterator for & NSEnumerator < ObjectType > { type IntoIter = Iter <'_ , ObjectType >; } impl < ObjectType : Message > IntoIterator for Retained < NSEnumerator < ObjectType >> { # [uses (new)] type IntoIter = IntoIter < ObjectType >; } }
};
}
