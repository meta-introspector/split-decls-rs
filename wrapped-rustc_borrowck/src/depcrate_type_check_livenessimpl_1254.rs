// Generated macro for impl_1254 (impl)
macro_rules! Depcrate_type_check_livenessimpl_1254 {
() => {
// Module: crate::type_check::liveness
// Provides: {"impl_1254"}
// Dependencies: {}
impl < 'a , 'tcx > LiveVariablesVisitor < 'a , 'tcx > { # [doc = " Some variable is \"regular live\" at `location` -- i.e., it may be used later. This means that"] # [doc = " all regions appearing in the type of `value` must be live at `location`."] fn record_regions_live_at < T > (& mut self , value : T , location : Location) where T : TypeVisitable < TyCtxt < 'tcx > > + Relate < TyCtxt < 'tcx > > , { debug ! ("record_regions_live_at(value={:?}, location={:?})" , value , location) ; self . tcx . for_each_free_region (& value , | live_region | { let live_region_vid = live_region . as_var () ; self . liveness_constraints . add_location (live_region_vid , location) ; }) ; if let Some (polonius_liveness) = self . polonius_liveness { polonius_liveness . record_live_region_variance (self . tcx , self . universal_regions , value) ; } } }
};
}
