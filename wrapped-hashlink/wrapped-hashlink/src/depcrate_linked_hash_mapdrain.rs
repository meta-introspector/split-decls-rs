// Generated macro for Drain (struct)
macro_rules! Depcrate_linked_hash_mapDrain {
() => {
// Module: crate::linked_hash_map
// Provides: {"Drain"}
// Dependencies: {}
pub struct Drain < 'a , K , V > { free : NonNull < Option < NonNull < Node < K , V > > > > , head : Option < NonNull < Node < K , V > > > , tail : Option < NonNull < Node < K , V > > > , remaining : usize , marker : PhantomData < (K , V , & 'a LinkedHashMap < K , V >) > , }
};
}
