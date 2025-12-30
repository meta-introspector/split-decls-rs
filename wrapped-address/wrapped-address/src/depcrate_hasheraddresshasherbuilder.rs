// Generated macro for AddressHasherBuilder (struct)
macro_rules! Depcrate_hasherAddressHasherBuilder {
() => {
// Module: crate::hasher
// Provides: {"AddressHasherBuilder"}
// Dependencies: {}
# [doc = " A builder for faster, but less collision resistant hasher for addresses."] # [doc = ""] # [doc = " Initializes `AddressHasher` instances that use an 8-byte"] # [doc = " slice of the address as the hash value. Should not be used when"] # [doc = " collisions might be used to mount DOS attacks."] # [doc = ""] # [doc = " Using this results in about 4x faster lookups in a typical hashmap."] # [derive (Clone)] pub struct AddressHasherBuilder { offset : usize , }
};
}
