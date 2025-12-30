// Generated macro for impl_818 (impl)
macro_rules! Depcrate_transitive_relationimpl_818 {
() => {
// Module: crate::transitive_relation
// Provides: {"impl_818"}
// Dependencies: {}
impl < T : Clone > Clone for TransitiveRelation < T > { fn clone (& self) -> Self { TransitiveRelation { builder : Frozen :: freeze (self . builder . deref () . clone ()) , closure : Frozen :: freeze (self . closure . deref () . clone ()) , } } }
};
}
