// Generated macro for __impl_slice_eq1 (macro)
macro_rules! Depcrate_collections_vec_deque_macros__impl_slice_eq1 {
() => {
// Module: crate::collections::vec_deque::macros
// Provides: {"__impl_slice_eq1"}
// Dependencies: {}
macro_rules ! __impl_slice_eq1 { ([$ ($ vars : tt) *] $ lhs : ty , $ rhs : ty , $ ($ constraints : tt) *) => { # [stable (feature = "vec_deque_partial_eq_slice" , since = "1.17.0")] impl < T , U , A : Allocator , $ ($ vars) *> PartialEq <$ rhs > for $ lhs where T : PartialEq < U >, $ ($ constraints) * { fn eq (& self , other : &$ rhs) -> bool { if self . len () != other . len () { return false ; } let (sa , sb) = self . as_slices () ; let (oa , ob) = other [..] . split_at (sa . len ()) ; sa == oa && sb == ob } } } }
};
}
