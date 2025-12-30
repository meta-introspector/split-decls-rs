// Generated macro for impl_921 (impl)
macro_rules! Depcrate_iter_onceimpl_921 {
() => {
// Module: crate::iter::once
// Provides: {"impl_921"}
// Dependencies: {}
impl < T : Send > IndexedParallelIterator for Once < T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { consumer . into_folder () . consume (self . item) . complete () } fn len (& self) -> usize { 1 } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { Some (self . item) . into_par_iter () . with_producer (callback) } }
};
}
