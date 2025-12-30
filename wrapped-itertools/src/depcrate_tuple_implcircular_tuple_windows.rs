// Generated macro for circular_tuple_windows (function)
macro_rules! Depcrate_tuple_implcircular_tuple_windows {
() => {
// Module: crate::tuple_impl
// Provides: {"circular_tuple_windows"}
// Dependencies: {}
pub fn circular_tuple_windows < I , T > (iter : I) -> CircularTupleWindows < I , T > where I : Iterator < Item = T :: Item > + Clone + ExactSizeIterator , T : TupleCollect + Clone , T :: Item : Clone , { let len = iter . len () ; let iter = tuple_windows (iter . cycle ()) ; CircularTupleWindows { iter , len } }
};
}
