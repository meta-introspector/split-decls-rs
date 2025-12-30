// Generated macro for impl_811 (impl)
macro_rules! Depcrate_strategy_filter_mapimpl_811 {
() => {
// Module: crate::strategy::filter_map
// Provides: {"impl_811"}
// Dependencies: {}
impl < S : Strategy , F : Fn (S :: Value) -> Option < O > , O : fmt :: Debug > Strategy for FilterMap < S , F > { type Tree = FilterMapValueTree < S :: Tree , F , O > ; type Value = O ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { loop { let val = self . source . new_tree (runner) ? ; if let Some (current) = (self . fun) (val . current ()) { return Ok (FilterMapValueTree :: new (val , & self . fun , current)) ; } else { runner . reject_local (self . whence . clone ()) ? ; } } } }
};
}
