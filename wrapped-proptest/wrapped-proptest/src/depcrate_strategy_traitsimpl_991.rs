// Generated macro for impl_991 (impl)
macro_rules! Depcrate_strategy_traitsimpl_991 {
() => {
// Module: crate::strategy::traits
// Provides: {"impl_991"}
// Dependencies: {}
impl < T : Strategy > Strategy for BoxedStrategyWrapper < T > where T :: Tree : 'static , { type Tree = Box < dyn ValueTree < Value = T :: Value > > ; type Value = T :: Value ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { Ok (Box :: new (self . 0 . new_tree (runner) ?)) } }
};
}
