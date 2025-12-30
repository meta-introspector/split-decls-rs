// Generated macro for impl_624 (impl)
macro_rules! Depcrate_displayimpl_624 {
() => {
// Module: crate::display
// Provides: {"impl_624"}
// Dependencies: {}
impl HirDisplay for TraitRef { fn hir_fmt (& self , f : & mut HirFormatter < '_ >) -> Result < () , HirDisplayError > { let trait_ = self . hir_trait_id () ; f . start_location_link (trait_ . into ()) ; write ! (f , "{}" , f . db . trait_signature (trait_) . name . display (f . db , f . edition ())) ? ; f . end_location_link () ; let substs = self . substitution . as_slice (Interner) ; hir_fmt_generics (f , & substs [1 ..] , None , substs [0] . ty (Interner)) } }
};
}
