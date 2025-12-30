// Generated macro for tuples (function)
macro_rules! Depcrate_tuple_impltuples {
() => {
// Module: crate::tuple_impl
// Provides: {"tuples"}
// Dependencies: {}
# [doc = " Create a new tuples iterator."] pub fn tuples < I , T > (iter : I) -> Tuples < I , T > where I : Iterator < Item = T :: Item > , T : HomogeneousTuple , { Tuples { iter : iter . fuse () , buf : Default :: default () , } }
};
}
