// Generated macro for TupleAppend (trait)
macro_rules! Depcrate_utilTupleAppend {
() => {
// Module: crate::util
// Provides: {"TupleAppend"}
// Dependencies: {}
# [doc = " Treats tuples as a list which can be appended to. e.g."] # [doc = " `(a,).tuple_append(b) == (a, b)`"] pub trait TupleAppend < T > { type Output ; fn tuple_append (self , right : T) -> Self :: Output ; }
};
}
