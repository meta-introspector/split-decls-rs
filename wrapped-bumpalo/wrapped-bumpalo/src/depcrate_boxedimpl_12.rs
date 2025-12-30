// Generated macro for impl_12 (impl)
macro_rules! Depcrate_boxedimpl_12 {
() => {
// Module: crate::boxed
// Provides: {"impl_12"}
// Dependencies: {}
impl < 'a , T : ? Sized + Ord > Ord for Box < 'a , T > { # [inline] fn cmp (& self , other : & Box < 'a , T >) -> Ordering { Ord :: cmp (& * * self , & * * other) } }
};
}
