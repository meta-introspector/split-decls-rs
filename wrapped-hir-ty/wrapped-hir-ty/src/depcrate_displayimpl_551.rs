// Generated macro for impl_551 (impl)
macro_rules! Depcrate_displayimpl_551 {
() => {
// Module: crate::display
// Provides: {"impl_551"}
// Dependencies: {}
impl < 'db > HirDisplay < 'db > for TraitRef < 'db > { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { let trait_ = self . def_id . 0 ; f . start_location_link (trait_ . into ()) ; write ! (f , "{}" , f . db . trait_signature (trait_) . name . display (f . db , f . edition ())) ? ; f . end_location_link () ; let substs = self . args . as_slice () ; hir_fmt_generic_args (f , & substs [1 ..] , None , Some (self . self_ty ())) } }
};
}
