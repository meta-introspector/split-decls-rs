// Generated macro for impl_799 (impl)
macro_rules! Depcrate_strategy_filterimpl_799 {
() => {
// Module: crate::strategy::filter
// Provides: {"impl_799"}
// Dependencies: {}
impl < S : Strategy , F : Fn (& S :: Value) -> bool > Strategy for Filter < S , F > { type Tree = Filter < S :: Tree , F > ; type Value = S :: Value ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { loop { let val = self . source . new_tree (runner) ? ; if ! (self . fun) (& val . current ()) { runner . reject_local (self . whence . clone ()) ? ; } else { return Ok (Filter { source : val , whence : self . whence . clone () , fun : Arc :: clone (& self . fun) , }) ; } } } }
};
}
