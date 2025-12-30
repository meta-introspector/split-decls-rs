// Generated macro for impl_148 (impl)
macro_rules! Depcrate_equivalentimpl_148 {
() => {
// Module: crate::equivalent
// Provides: {"impl_148"}
// Dependencies: {}
impl < K : ? Sized , Q : ? Sized > Comparable < Q > for K where K : Borrow < Q > , Q : Ord , { # [inline] fn compare (& self , key : & Q) -> Ordering { Ord :: cmp (self . borrow () , key) } }
};
}
