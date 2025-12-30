// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl Hash { pub const fn new_from_array (hash_array : [u8 ; HASH_BYTES]) -> Self { Self (hash_array) } # [doc = " unique Hash for tests and benchmarks."] # [cfg (feature = "atomic")] pub fn new_unique () -> Self { use solana_atomic_u64 :: AtomicU64 ; static I : AtomicU64 = AtomicU64 :: new (1) ; let mut b = [0u8 ; HASH_BYTES] ; let i = I . fetch_add (1) ; b [0 .. 8] . copy_from_slice (& i . to_le_bytes ()) ; Self :: new_from_array (b) } pub const fn to_bytes (& self) -> [u8 ; HASH_BYTES] { self . 0 } pub const fn as_bytes (& self) -> & [u8 ; HASH_BYTES] { & self . 0 } }
};
}
