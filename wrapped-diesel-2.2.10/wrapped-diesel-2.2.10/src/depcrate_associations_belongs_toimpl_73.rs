// Generated macro for impl_73 (impl)
macro_rules! Depcrate_associations_belongs_toimpl_73 {
() => {
// Module: crate::associations::belongs_to
// Provides: {"impl_73"}
// Dependencies: {}
impl < 'a , Parent , Child > BelongingToDsl < & 'a Vec < Parent > > for Child where Child : BelongingToDsl < & 'a [Parent] > , { type Output = Child :: Output ; fn belonging_to (parents : & 'a Vec < Parent >) -> Self :: Output { Self :: belonging_to (& * * parents) } }
};
}
