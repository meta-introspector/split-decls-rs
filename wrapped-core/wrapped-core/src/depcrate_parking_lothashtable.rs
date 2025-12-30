// Generated macro for HashTable (struct)
macro_rules! Depcrate_parking_lotHashTable {
() => {
// Module: crate::parking_lot
// Provides: {"HashTable"}
// Dependencies: {}
struct HashTable { entries : Box < [Bucket] > , hash_bits : u32 , _prev : * const HashTable , }
};
}
