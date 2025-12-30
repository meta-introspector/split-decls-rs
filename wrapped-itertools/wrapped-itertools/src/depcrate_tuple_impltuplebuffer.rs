// Generated macro for TupleBuffer (struct)
macro_rules! Depcrate_tuple_implTupleBuffer {
() => {
// Module: crate::tuple_impl
// Provides: {"TupleBuffer"}
// Dependencies: {}
# [doc = " An iterator over a incomplete tuple."] # [doc = ""] # [doc = " See [`.tuples()`](crate::Itertools::tuples) and"] # [doc = " [`Tuples::into_buffer()`]."] # [derive (Clone , Debug)] pub struct TupleBuffer < T > where T : HomogeneousTuple , { cur : usize , buf : T :: Buffer , }
};
}
