// Generated macro for impl_98 (impl)
macro_rules! Depcrate_key_hashimpl_98 {
() => {
// Module: crate::key::hash
// Provides: {"impl_98"}
// Dependencies: {}
impl Fnv1a64Hasher { const BASIS : u64 = 0xcbf2_9ce4_8422_2325 ; const PRIME : u64 = 0x0000_0100_0000_01b3 ; # [doc = " Create a new hasher with the default basis as state contents"] pub fn new () -> Self { Self { state : Self :: BASIS } } # [doc = " Calculate the hash for each of the given data bytes"] pub fn update (& mut self , data : & [u8]) { for b in data { let ext = u64 :: from (* b) ; self . state ^= ext ; self . state = self . state . wrapping_mul (Self :: PRIME) ; } } # [doc = " Extract the current state for finalizing the hash"] pub fn digest (self) -> u64 { self . state } # [doc = " Same as digest but as bytes"] pub fn digest_bytes (self) -> [u8 ; 8] { self . digest () . to_le_bytes () } }
};
}
