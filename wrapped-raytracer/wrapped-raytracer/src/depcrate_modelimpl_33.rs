// Generated macro for impl_33 (impl)
macro_rules! Depcrate_modelimpl_33 {
() => {
// Module: crate::model
// Provides: {"impl_33"}
// Dependencies: {}
impl Model for Vec < Box < dyn Model > > { fn hit (& self , r : & Ray) -> Option < Hit > { let mut best = None ; for child in self { if let Some (hit) = child . hit (r) { match best { None => best = Some (hit) , Some (prev) => { if hit . t < prev . t { best = Some (hit) } } } } } best } }
};
}
