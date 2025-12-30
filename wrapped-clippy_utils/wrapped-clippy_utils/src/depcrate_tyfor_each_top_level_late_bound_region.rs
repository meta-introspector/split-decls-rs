// Generated macro for for_each_top_level_late_bound_region (function)
macro_rules! Depcrate_tyfor_each_top_level_late_bound_region {
() => {
// Module: crate::ty
// Provides: {"for_each_top_level_late_bound_region"}
// Dependencies: {}
pub fn for_each_top_level_late_bound_region < B > (ty : Ty < '_ > , f : impl FnMut (BoundRegion) -> ControlFlow < B > ,) -> ControlFlow < B > { struct V < F > { index : u32 , f : F , } impl < 'tcx , B , F : FnMut (BoundRegion) -> ControlFlow < B > > TypeVisitor < TyCtxt < 'tcx > > for V < F > { type Result = ControlFlow < B > ; fn visit_region (& mut self , r : Region < 'tcx >) -> Self :: Result { if let RegionKind :: ReBound (BoundVarIndexKind :: Bound (idx) , bound) = r . kind () && idx . as_u32 () == self . index { (self . f) (bound) } else { ControlFlow :: Continue (()) } } fn visit_binder < T : TypeVisitable < TyCtxt < 'tcx > > > (& mut self , t : & Binder < 'tcx , T >) -> Self :: Result { self . index += 1 ; let res = t . super_visit_with (self) ; self . index -= 1 ; res } } ty . visit_with (& mut V { index : 0 , f }) }
};
}
