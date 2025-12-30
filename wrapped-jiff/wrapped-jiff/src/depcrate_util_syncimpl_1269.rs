// Generated macro for impl_1269 (impl)
macro_rules! Depcrate_util_syncimpl_1269 {
() => {
// Module: crate::util::sync
// Provides: {"impl_1269"}
// Dependencies: {}
# [cfg (not (feature = "alloc"))] impl < T > Arc < T > { pub (crate) fn new (t : T) -> Arc < T > { Arc (t) } pub (crate) fn get_mut (this : & mut Arc < T >) -> Option < & mut T > { Some (& mut this . 0) } }
};
}
