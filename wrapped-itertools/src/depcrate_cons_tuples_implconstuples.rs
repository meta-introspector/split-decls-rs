// Generated macro for ConsTuples (type)
macro_rules! Depcrate_cons_tuples_implConsTuples {
() => {
// Module: crate::cons_tuples_impl
// Provides: {"ConsTuples"}
// Dependencies: {}
# [doc = " An iterator that maps an iterator of tuples like"] # [doc = " `((A, B), C)` to an iterator of `(A, B, C)`."] # [doc = ""] # [doc = " Used by the `iproduct!()` macro."] pub type ConsTuples < I > = MapSpecialCase < I , ConsTuplesFn > ;
};
}
