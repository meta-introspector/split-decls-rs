// Generated macro for impl_cons_iter (macro)
macro_rules! Depcrate_cons_tuples_implimpl_cons_iter {
() => {
// Module: crate::cons_tuples_impl
// Provides: {"impl_cons_iter"}
// Dependencies: {}
macro_rules ! impl_cons_iter (($ _A : ident , $ _B : ident ,) => () ; ($ A : ident , $ ($ B : ident ,) *) => (impl_cons_iter ! ($ ($ B ,) *) ; # [allow (non_snake_case)] impl <$ ($ B) ,*, X > MapSpecialCaseFn < (($ ($ B ,) *) , X) > for ConsTuplesFn { type Out = ($ ($ B ,) * X ,) ; fn call (& mut self , (($ ($ B ,) *) , X) : (($ ($ B ,) *) , X)) -> Self :: Out { ($ ($ B ,) * X ,) } }) ;) ;
};
}
