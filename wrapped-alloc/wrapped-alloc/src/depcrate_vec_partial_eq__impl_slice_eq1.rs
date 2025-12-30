// Generated macro for __impl_slice_eq1 (macro)
macro_rules! Depcrate_vec_partial_eq__impl_slice_eq1 {
() => {
// Module: crate::vec::partial_eq
// Provides: {"__impl_slice_eq1"}
// Dependencies: {}
macro_rules ! __impl_slice_eq1 { ([$ ($ vars : tt) *] $ lhs : ty , $ rhs : ty $ (where $ ty : ty : $ bound : ident) ?, # [$ stability : meta]) => { # [$ stability] impl < T , U , $ ($ vars) *> PartialEq <$ rhs > for $ lhs where T : PartialEq < U >, $ ($ ty : $ bound) ? { # [inline] fn eq (& self , other : &$ rhs) -> bool { self [..] == other [..] } # [inline] fn ne (& self , other : &$ rhs) -> bool { self [..] != other [..] } } } }
};
}
