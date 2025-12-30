// Generated macro for impl_817 (impl)
macro_rules! Depcrate_transitive_relationimpl_817 {
() => {
// Module: crate::transitive_relation
// Provides: {"impl_817"}
// Dependencies: {}
impl < T > Deref for TransitiveRelation < T > { type Target = Frozen < TransitiveRelationBuilder < T > > ; fn deref (& self) -> & Self :: Target { & self . builder } }
};
}
