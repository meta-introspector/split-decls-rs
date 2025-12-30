// Generated macro for impl_177 (impl)
macro_rules! Depcrate_stackvecimpl_177 {
() => {
// Module: crate::stackvec
// Provides: {"impl_177"}
// Dependencies: {}
impl PartialEq for StackVec { # [inline] # [allow (clippy :: op_ref)] fn eq (& self , other : & Self) -> bool { use core :: ops :: Deref ; self . len () == other . len () && self . deref () == other . deref () } }
};
}
