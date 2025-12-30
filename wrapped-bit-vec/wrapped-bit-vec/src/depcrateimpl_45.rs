// Generated macro for impl_45 (impl)
macro_rules! Depcrateimpl_45 {
() => {
// Module: crate
// Provides: {"impl_45"}
// Dependencies: {}
impl < B : BitBlock > Ord for BitVec < B > { # [inline] fn cmp (& self , other : & Self) -> Ordering { self . ensure_invariant () ; debug_assert ! (other . is_last_block_fixed ()) ; let mut a = self . iter () ; let mut b = other . iter () ; loop { match (a . next () , b . next ()) { (Some (x) , Some (y)) => match x . cmp (& y) { Ordering :: Equal => { } otherwise => return otherwise , } , (None , None) => return Ordering :: Equal , (None , _) => return Ordering :: Less , (_ , None) => return Ordering :: Greater , } } } }
};
}
