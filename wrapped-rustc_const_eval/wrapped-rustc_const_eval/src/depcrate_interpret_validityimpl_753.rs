// Generated macro for impl_753 (impl)
macro_rules! Depcrate_interpret_validityimpl_753 {
() => {
// Module: crate::interpret::validity
// Provides: {"impl_753"}
// Dependencies: {}
impl < T : Clone + Eq + Hash + std :: fmt :: Debug , PATH : Default > RefTracking < T , PATH > { pub fn empty () -> Self { RefTracking { seen : FxHashSet :: default () , todo : vec ! [] } } pub fn new (val : T) -> Self { let mut ref_tracking_for_consts = RefTracking { seen : FxHashSet :: default () , todo : vec ! [(val . clone () , PATH :: default ())] } ; ref_tracking_for_consts . seen . insert (val) ; ref_tracking_for_consts } pub fn next (& mut self) -> Option < (T , PATH) > { self . todo . pop () } fn track (& mut self , val : T , path : impl FnOnce () -> PATH) { if self . seen . insert (val . clone ()) { trace ! ("Recursing below ptr {:#?}" , val) ; let path = path () ; self . todo . push ((val , path)) ; } } }
};
}
