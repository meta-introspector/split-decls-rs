// Generated macro for impl_960 (impl)
macro_rules! Depcrate_strategy_shuffleimpl_960 {
() => {
// Module: crate::strategy::shuffle
// Provides: {"impl_960"}
// Dependencies: {}
impl < S : Strategy > Strategy for Shuffle < S > where S :: Value : Shuffleable , { type Tree = ShuffleValueTree < S :: Tree > ; type Value = S :: Value ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { let rng = runner . new_rng () ; self . 0 . new_tree (runner) . map (| inner | ShuffleValueTree { inner , rng , dist : Cell :: new (None) , simplifying_inner : false , }) } }
};
}
