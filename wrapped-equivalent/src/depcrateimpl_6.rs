// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl < Q : ? Sized , K : ? Sized > Comparable < K > for Q where Q : Ord , K : Borrow < Q > , { # [inline] fn compare (& self , key : & K) -> Ordering { Ord :: cmp (self , key . borrow ()) } }
};
}
