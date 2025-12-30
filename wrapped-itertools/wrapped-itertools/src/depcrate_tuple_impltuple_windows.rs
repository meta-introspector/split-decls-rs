// Generated macro for tuple_windows (function)
macro_rules! Depcrate_tuple_impltuple_windows {
() => {
// Module: crate::tuple_impl
// Provides: {"tuple_windows"}
// Dependencies: {}
# [doc = " Create a new tuple windows iterator."] pub fn tuple_windows < I , T > (iter : I) -> TupleWindows < I , T > where I : Iterator < Item = T :: Item > , T : HomogeneousTuple , T :: Item : Clone , { TupleWindows { last : None , iter } }
};
}
