// Generated macro for impl_1070 (impl)
macro_rules! Depcrate_strategy_staticsimpl_1070 {
() => {
// Module: crate::strategy::statics
// Provides: {"impl_1070"}
// Dependencies: {}
impl < S : Strategy , F : FilterFn < S :: Value > + Clone > Strategy for Filter < S , F > { type Tree = Filter < S :: Tree , F > ; type Value = S :: Value ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { loop { let val = self . source . new_tree (runner) ? ; if ! self . fun . apply (& val . current ()) { runner . reject_local (self . whence . clone ()) ? ; } else { return Ok (Filter { source : val , whence : "unused" . into () , fun : self . fun . clone () , }) ; } } } }
};
}
