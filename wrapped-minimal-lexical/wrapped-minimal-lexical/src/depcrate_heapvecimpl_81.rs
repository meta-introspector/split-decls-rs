// Generated macro for impl_81 (impl)
macro_rules! Depcrate_heapvecimpl_81 {
() => {
// Module: crate::heapvec
// Provides: {"impl_81"}
// Dependencies: {}
impl PartialEq for HeapVec { # [inline] # [allow (clippy :: op_ref)] fn eq (& self , other : & Self) -> bool { use core :: ops :: Deref ; self . len () == other . len () && self . deref () == other . deref () } }
};
}
