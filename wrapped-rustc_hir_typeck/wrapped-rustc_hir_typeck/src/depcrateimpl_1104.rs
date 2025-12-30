// Generated macro for impl_1104 (impl)
macro_rules! Depcrateimpl_1104 {
() => {
// Module: crate
// Provides: {"impl_1104"}
// Dependencies: {}
impl < 'tcx > EnclosingBreakables < 'tcx > { fn find_breakable (& mut self , target_id : HirId) -> & mut BreakableCtxt < 'tcx > { self . opt_find_breakable (target_id) . unwrap_or_else (| | { bug ! ("could not find enclosing breakable with id {}" , target_id) ; }) } fn opt_find_breakable (& mut self , target_id : HirId) -> Option < & mut BreakableCtxt < 'tcx > > { match self . by_id . get (& target_id) { Some (ix) => Some (& mut self . stack [* ix]) , None => None , } } }
};
}
