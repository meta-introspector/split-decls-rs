// Generated macro for impl_837 (impl)
macro_rules! Depcrate_strategy_flattenimpl_837 {
() => {
// Module: crate::strategy::flatten
// Provides: {"impl_837"}
// Dependencies: {}
impl < S : Strategy , R : Strategy , F : Fn (S :: Value) -> R > Strategy for IndFlattenMap < S , F > { type Tree = crate :: tuple :: TupleValueTree < (S :: Tree , R :: Tree) > ; type Value = (S :: Value , R :: Value) ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { let left = self . source . new_tree (runner) ? ; let right_source = (self . fun) (left . current ()) ; let right = right_source . new_tree (runner) ? ; Ok (crate :: tuple :: TupleValueTree :: new ((left , right))) } }
};
}
