// Generated macro for tuple_union (macro)
macro_rules! Depcrate_strategy_unionstuple_union {
() => {
// Module: crate::strategy::unions
// Provides: {"tuple_union"}
// Dependencies: {}
macro_rules ! tuple_union { ($ ($ gen : ident $ ix : tt) *) => { impl < A : Strategy , $ ($ gen : Strategy < Value = A :: Value >) ,*> Strategy for TupleUnion < (WA < A >, $ (WA <$ gen >) ,*) > { type Tree = TupleUnionValueTree < (LazyValueTree < A >, $ (Option < LazyValueTree <$ gen >>) ,*) >; type Value = A :: Value ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { let weights = [((self . 0) . 0) . 0 , $ (((self . 0) .$ ix) . 0) ,*] ; let pick = pick_weighted (runner , weights . iter () . cloned () , weights . iter () . cloned ()) ; Ok (TupleUnionValueTree { options : (if 0 == pick { LazyValueTree :: new_initialized (((self . 0) . 0) . 1 . new_tree (runner) ?) } else { LazyValueTree :: new (Arc :: clone (& ((self . 0) . 0) . 1) , runner) } , $ (if $ ix == pick { Some (LazyValueTree :: new_initialized (((self . 0) .$ ix) . 1 . new_tree (runner) ?)) } else if $ ix < pick { Some (LazyValueTree :: new (Arc :: clone (& ((self . 0) .$ ix) . 1) , runner)) } else { None }) ,*) , pick : pick , min_pick : 0 , prev_pick : None , }) } } } }
};
}
