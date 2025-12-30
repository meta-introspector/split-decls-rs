// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl Registry { pub const fn new () -> Self { Registry { head : AtomicPtr :: new (ptr :: null_mut ()) , } } unsafe fn submit (& 'static self , new : & 'static Node) { # [cfg (target_family = "wasm")] if new . initialized . swap (true , Ordering :: Relaxed) { return ; } let mut head = self . head . load (Ordering :: Relaxed) ; loop { unsafe { * new . next . get () = head . as_ref () ; } let new_ptr = new as * const Node as * mut Node ; match self . head . compare_exchange (head , new_ptr , Ordering :: Release , Ordering :: Relaxed) { Ok (_) => return , Err (prev) => head = prev , } } } }
};
}
