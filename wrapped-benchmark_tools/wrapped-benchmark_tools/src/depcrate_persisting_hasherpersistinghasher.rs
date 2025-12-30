// Generated macro for PersistingHasher (struct)
macro_rules! Depcrate_persisting_hasherPersistingHasher {
() => {
// Module: crate::persisting_hasher
// Provides: {"PersistingHasher"}
// Dependencies: {}
pub struct PersistingHasher { # [doc = " Used to compute a hash"] hash : u64 , # [doc = " File to write data out to"] out : Arc < Mutex < BufWriter < File > > > , }
};
}
