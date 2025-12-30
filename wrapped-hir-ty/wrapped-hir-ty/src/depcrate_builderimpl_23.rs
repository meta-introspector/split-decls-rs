// Generated macro for impl_23 (impl)
macro_rules! Depcrate_builderimpl_23 {
() => {
// Module: crate::builder
// Provides: {"impl_23"}
// Dependencies: {}
impl < T : HasInterner < Interner = Interner > + TypeFoldable < Interner > > TyBuilder < Binders < T > > { pub fn build (self) -> T { let (b , subst) = self . build_internal () ; b . substitute (Interner , & subst) } }
};
}
