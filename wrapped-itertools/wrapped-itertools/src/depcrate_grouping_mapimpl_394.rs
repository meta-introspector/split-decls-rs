// Generated macro for impl_394 (impl)
macro_rules! Depcrate_grouping_mapimpl_394 {
() => {
// Module: crate::grouping_map
// Provides: {"impl_394"}
// Dependencies: {}
impl < V , K , F : FnMut (& V) -> K > MapSpecialCaseFn < V > for GroupingMapFn < F > { type Out = (K , V) ; fn call (& mut self , v : V) -> Self :: Out { ((self . 0) (& v) , v) } }
};
}
