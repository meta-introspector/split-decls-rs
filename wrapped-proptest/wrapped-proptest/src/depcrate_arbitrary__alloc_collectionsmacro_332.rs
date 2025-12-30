// Generated macro for macro_332 (macro)
macro_rules! Depcrate_arbitrary__alloc_collectionsmacro_332 {
() => {
// Module: crate::arbitrary::_alloc::collections
// Provides: {"macro_332"}
// Dependencies: {}
lift1 ! (['static] Bound < A >; base => { let base = Rc :: new (base) ; prop_oneof ! [2 => base . clone () . prop_map (Bound :: Included) , 2 => base . prop_map (Bound :: Excluded) , 1 => LazyJustFn :: new (|| Bound :: Unbounded) ,] }) ;
};
}
