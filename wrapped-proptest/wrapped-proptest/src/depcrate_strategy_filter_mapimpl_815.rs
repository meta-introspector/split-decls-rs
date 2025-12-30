// Generated macro for impl_815 (impl)
macro_rules! Depcrate_strategy_filter_mapimpl_815 {
() => {
// Module: crate::strategy::filter_map
// Provides: {"impl_815"}
// Dependencies: {}
impl < V : ValueTree , F : Fn (V :: Value) -> Option < O > , O > FilterMapValueTree < V , F , O > { fn new (source : V , fun : & Arc < F > , current : O) -> Self { Self { source , current : Cell :: new (Some (current)) , fun : Arc :: clone (fun) , } } fn fresh_current (& self) -> O { (self . fun) (self . source . current ()) . expect ("internal logic error; this is a bug!") } fn ensure_acceptable (& mut self) { loop { if let Some (current) = (self . fun) (self . source . current ()) { self . current = Cell :: new (Some (current)) ; break ; } else if ! self . source . complicate () { panic ! ("Unable to complicate filtered strategy \
                     back into acceptable value") ; } } } }
};
}
