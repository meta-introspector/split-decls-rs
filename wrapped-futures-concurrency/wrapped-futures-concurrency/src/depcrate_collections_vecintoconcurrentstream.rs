// Generated macro for IntoConcurrentStream (struct)
macro_rules! Depcrate_collections_vecIntoConcurrentStream {
() => {
// Module: crate::collections::vec
// Provides: {"IntoConcurrentStream"}
// Dependencies: {}
# [doc = " Concurrent async iterator that moves out of a vector."] # [derive (Debug)] pub struct IntoConcurrentStream < T > (FromStream < FromIter < alloc :: vec :: IntoIter < T > > >) ;
};
}
