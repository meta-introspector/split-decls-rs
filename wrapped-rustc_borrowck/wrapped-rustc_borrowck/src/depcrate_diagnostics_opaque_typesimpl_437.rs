// Generated macro for impl_437 (impl)
macro_rules! Depcrate_diagnostics_opaque_typesimpl_437 {
() => {
// Module: crate::diagnostics::opaque_types
// Provides: {"impl_437"}
// Dependencies: {}
impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for CheckExplicitRegionMentionAndCollectGenerics < 'tcx > { type Result = ControlFlow < () , () > ; fn visit_ty (& mut self , ty : Ty < 'tcx >) -> Self :: Result { match * ty . kind () { ty :: Alias (ty :: Opaque , opaque) => { if self . seen_opaques . insert (opaque . def_id) { for (bound , _) in self . tcx . explicit_item_bounds (opaque . def_id) . iter_instantiated_copied (self . tcx , opaque . args) { bound . visit_with (self) ? ; } } ControlFlow :: Continue (()) } _ => ty . super_visit_with (self) , } } fn visit_region (& mut self , r : ty :: Region < 'tcx >) -> Self :: Result { match r . kind () { ty :: ReEarlyParam (param) => { if param . index as usize == self . offending_region_idx { ControlFlow :: Break (()) } else { self . seen_lifetimes . insert (self . generics . region_param (param , self . tcx) . def_id) ; ControlFlow :: Continue (()) } } _ => ControlFlow :: Continue (()) , } } }
};
}
