// Generated macro for impl_147 (impl)
macro_rules! Depcrate_hashimpl_147 {
() => {
// Module: crate::hash
// Provides: {"impl_147"}
// Dependencies: {}
impl < T : Hash > fmt :: Display for ShortHash < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { static HASHED : AtomicBool = AtomicBool :: new (false) ; static HASH : AtomicUsize = AtomicUsize :: new (0) ; if ! HASHED . load (SeqCst) { let mut h = DefaultHasher :: new () ; env :: var ("CARGO_PKG_NAME") . expect ("should have CARGO_PKG_NAME env var") . hash (& mut h) ; env :: var ("CARGO_PKG_VERSION") . expect ("should have CARGO_PKG_VERSION env var") . hash (& mut h) ; HASH . store (h . finish () as usize , SeqCst) ; HASHED . store (true , SeqCst) ; } let mut h = DefaultHasher :: new () ; HASH . load (SeqCst) . hash (& mut h) ; self . 0 . hash (& mut h) ; write ! (f , "{:016x}" , h . finish ()) } }
};
}
