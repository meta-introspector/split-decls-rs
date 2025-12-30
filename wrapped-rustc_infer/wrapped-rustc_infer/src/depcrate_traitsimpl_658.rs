// Generated macro for impl_658 (impl)
macro_rules! Depcrate_traitsimpl_658 {
() => {
// Module: crate::traits
// Provides: {"impl_658"}
// Dependencies: {}
impl < 'tcx > PolyTraitObligation < 'tcx > { pub fn polarity (& self) -> ty :: PredicatePolarity { self . predicate . skip_binder () . polarity } pub fn self_ty (& self) -> ty :: Binder < 'tcx , Ty < 'tcx > > { self . predicate . map_bound (| p | p . self_ty ()) } }
};
}
