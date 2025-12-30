// Generated macro for implementation (module)
macro_rules! Depcrateimplementation {
() => {
// Module: crate
// Provides: {"implementation"}
// Dependencies: {}
# [cfg (not (target_pointer_width = "64"))] mod implementation { use parking_lot :: { const_mutex , Mutex } ; pub struct AtomicU64 (Mutex < u64 >) ; impl AtomicU64 { pub const fn new (initial : u64) -> Self { Self (const_mutex (initial)) } pub fn fetch_add (& self , v : u64) -> u64 { let mut lock = self . 0 . lock () ; let i = * lock ; * lock = i + v ; i } } }
};
}
