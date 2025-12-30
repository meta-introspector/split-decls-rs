// Generated macro for tuple (macro)
macro_rules! Depcrate_tupletuple {
() => {
// Module: crate::tuple
// Provides: {"tuple"}
// Dependencies: {}
macro_rules ! tuple { ($ ($ fld : tt : $ typ : ident) ,*) => { impl <$ ($ typ : Strategy) ,*> Strategy for ($ ($ typ ,) *) { type Tree = TupleValueTree < ($ ($ typ :: Tree ,) *) >; type Value = ($ ($ typ :: Value ,) *) ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { let values = ($ (self .$ fld . new_tree (runner) ?,) *) ; Ok (TupleValueTree :: new (values)) } } impl <$ ($ typ : ValueTree) ,*> ValueTree for TupleValueTree < ($ ($ typ ,) *) > { type Value = ($ ($ typ :: Value ,) *) ; fn current (& self) -> Self :: Value { ($ (self . tree .$ fld . current () ,) *) } fn simplify (& mut self) -> bool { $ (if $ fld == self . shrinker { if self . tree .$ fld . simplify () { self . prev_shrinker = Some (self . shrinker) ; return true ; } else { self . shrinker += 1 ; } }) * false } fn complicate (& mut self) -> bool { if let Some (shrinker) = self . prev_shrinker { $ (if $ fld == shrinker { if self . tree .$ fld . complicate () { self . shrinker = shrinker ; return true ; } else { self . prev_shrinker = None ; return false ; } }) * } false } } } }
};
}
