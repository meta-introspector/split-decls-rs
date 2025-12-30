// Generated macro for impl_566 (impl)
macro_rules! Depcrate_visitimpl_566 {
() => {
// Module: crate::visit
// Provides: {"impl_566"}
// Dependencies: {}
impl BoundKind { pub fn descr (self) -> & 'static str { match self { BoundKind :: Bound => "bounds" , BoundKind :: Impl => "`impl Trait`" , BoundKind :: TraitObject => "`dyn` trait object bounds" , BoundKind :: SuperTraits => "supertrait bounds" , } } }
};
}
