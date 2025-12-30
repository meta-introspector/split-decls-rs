// Generated macro for impl_1319 (impl)
macro_rules! Depcrate_optionimpl_1319 {
() => {
// Module: crate::option
// Provides: {"impl_1319"}
// Dependencies: {}
impl < T : Send > IndexedParallelIterator for IntoIter < T > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { let mut folder = consumer . into_folder () ; if let Some (item) = self . opt { folder = folder . consume (item) ; } folder . complete () } fn len (& self) -> usize { match self . opt { Some (_) => 1 , None => 0 , } } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { callback . callback (OptionProducer { opt : self . opt }) } }
};
}
