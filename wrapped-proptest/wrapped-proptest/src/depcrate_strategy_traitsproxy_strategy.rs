// Generated macro for proxy_strategy (macro)
macro_rules! Depcrate_strategy_traitsproxy_strategy {
() => {
// Module: crate::strategy::traits
// Provides: {"proxy_strategy"}
// Dependencies: {}
macro_rules ! proxy_strategy { ($ typ : ty $ (, $ lt : tt) *) => { impl <$ ($ lt ,) * S : Strategy + ? Sized > Strategy for $ typ { type Tree = S :: Tree ; type Value = S :: Value ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { (** self) . new_tree (runner) } } } ; }
};
}
