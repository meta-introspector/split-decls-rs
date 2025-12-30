// Generated macro for impl_989 (impl)
macro_rules! Depcrate_strategy_traitsimpl_989 {
() => {
// Module: crate::strategy::traits
// Provides: {"impl_989"}
// Dependencies: {}
impl < T : fmt :: Debug > Strategy for SBoxedStrategy < T > { type Tree = BoxedVT < T > ; type Value = T ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { self . 0 . new_tree (runner) } fn sboxed (self) -> SBoxedStrategy < Self :: Value > where Self : Sized + Send + Sync + 'static , { self } fn boxed (self) -> BoxedStrategy < Self :: Value > where Self : Sized + 'static , { BoxedStrategy (self . 0) } }
};
}
