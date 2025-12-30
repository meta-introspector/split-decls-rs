// Generated macro for Drain (struct)
macro_rules! Depcrate_collections_hash_mapDrain {
() => {
// Module: crate::collections::hash_map
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " Draining parallel iterator that moves out of a hash map,"] # [doc = " but keeps the total capacity."] # [derive (Debug)] pub struct Drain < 'a , K , V > { inner : vec :: IntoIter < (K , V) > , marker : PhantomData < & 'a mut HashMap < K , V > > , }
};
}
