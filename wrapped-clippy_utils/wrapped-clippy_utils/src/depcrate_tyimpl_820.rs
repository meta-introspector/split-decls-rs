// Generated macro for impl_820 (impl)
macro_rules! Depcrate_tyimpl_820 {
() => {
// Module: crate::ty
// Provides: {"impl_820"}
// Dependencies: {}
impl AdtVariantInfo { # [doc = " Returns ADT variants ordered by size"] pub fn new < 'tcx > (cx : & LateContext < 'tcx > , adt : AdtDef < 'tcx > , subst : GenericArgsRef < 'tcx >) -> Vec < Self > { let mut variants_size = adt . variants () . iter () . enumerate () . map (| (i , variant) | { let mut fields_size = variant . fields . iter () . enumerate () . map (| (i , f) | (i , approx_ty_size (cx , f . ty (cx . tcx , subst)))) . collect :: < Vec < _ > > () ; fields_size . sort_by (| (_ , a_size) , (_ , b_size) | a_size . cmp (b_size)) ; Self { ind : i , size : fields_size . iter () . map (| (_ , size) | size) . sum () , fields_size , } }) . collect :: < Vec < _ > > () ; variants_size . sort_by (| a , b | b . size . cmp (& a . size)) ; variants_size } }
};
}
