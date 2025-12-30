// Generated macro for Drain (struct)
macro_rules! Depcrate_collections_hash_setDrain {
() => {
// Module: crate::collections::hash_set
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " Draining parallel iterator that moves out of a hash set,"] # [doc = " but keeps the total capacity."] # [derive (Debug)] pub struct Drain < 'a , T > { inner : vec :: IntoIter < T > , marker : PhantomData < & 'a mut HashSet < T > > , }
};
}
