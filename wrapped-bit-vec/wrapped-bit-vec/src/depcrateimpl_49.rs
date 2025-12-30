// Generated macro for impl_49 (impl)
macro_rules! Depcrateimpl_49 {
() => {
// Module: crate
// Provides: {"impl_49"}
// Dependencies: {}
impl < B : BitBlock > cmp :: PartialEq for BitVec < B > { # [inline] fn eq (& self , other : & Self) -> bool { if self . nbits != other . nbits { self . ensure_invariant () ; other . ensure_invariant () ; return false ; } self . blocks () . zip (other . blocks ()) . all (| (w1 , w2) | w1 == w2) } }
};
}
