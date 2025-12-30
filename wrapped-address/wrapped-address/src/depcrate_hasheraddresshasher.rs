// Generated macro for AddressHasher (struct)
macro_rules! Depcrate_hasherAddressHasher {
() => {
// Module: crate::hasher
// Provides: {"AddressHasher"}
// Dependencies: {}
# [doc = " A faster, but less collision resistant hasher for addresses."] # [doc = ""] # [doc = " Specialized hasher that uses a random 8 bytes subslice of the"] # [doc = " address as the hash value. Should not be used when collisions"] # [doc = " might be used to mount DOS attacks."] # [doc = ""] # [doc = " Using this results in about 4x faster lookups in a typical hashmap."] # [derive (Default)] pub struct AddressHasher { offset : usize , state : u64 , }
};
}
