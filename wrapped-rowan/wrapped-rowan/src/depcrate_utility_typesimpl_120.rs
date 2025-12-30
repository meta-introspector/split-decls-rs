// Generated macro for impl_120 (impl)
macro_rules! Depcrate_utility_typesimpl_120 {
() => {
// Module: crate::utility_types
// Provides: {"impl_120"}
// Dependencies: {}
impl < T > WalkEvent < T > { pub fn map < F : FnOnce (T) -> U , U > (self , f : F) -> WalkEvent < U > { match self { WalkEvent :: Enter (it) => WalkEvent :: Enter (f (it)) , WalkEvent :: Leave (it) => WalkEvent :: Leave (f (it)) , } } }
};
}
