// Generated macro for sync (module)
macro_rules! Depcratesync {
() => {
// Module: crate
// Provides: {"sync"}
// Dependencies: {}
# [doc = " thread-safe types"] pub mod sync { # [doc = " A map for associating data with object ids in a thread-safe fashion. It should scale well up to 256 threads."] pub struct ObjectIdMap < V > { # [doc = " Sharing is done by the first byte of the incoming object id."] shards : [parking_lot :: Mutex < super :: HashMap < gix_hash :: ObjectId , V > > ; 256] , } impl < V > Default for ObjectIdMap < V > { fn default () -> Self { Self { shards : std :: array :: from_fn (| _ | parking_lot :: Mutex :: new (super :: HashMap :: default ())) , } } } # [doc = " access and modifications - we only implement what's used within the `gix-*` ecosystem."] impl < V > ObjectIdMap < V > { # [doc = " Insert `value` at `key` and return `None` if it's the first value at that location, or `Some(previous-value)`"] # [doc = " if `key` was already set."] pub fn insert (& self , key : gix_hash :: ObjectId , value : V) -> Option < V > { self . shards [key . as_slice () [0] as usize] . lock () . insert (key , value) } } }
};
}
