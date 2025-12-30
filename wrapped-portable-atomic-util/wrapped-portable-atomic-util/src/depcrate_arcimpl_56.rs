// Generated macro for impl_56 (impl)
macro_rules! Depcrate_arcimpl_56 {
() => {
// Module: crate::arc
// Provides: {"impl_56"}
// Dependencies: {}
impl < T : ? Sized + Ord > Ord for Arc < T > { # [doc = " Comparison for two `Arc`s."] # [doc = ""] # [doc = " The two are compared by calling `cmp()` on their inner values."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = " use std::cmp::Ordering;"] # [doc = ""] # [doc = " let five = Arc::new(5);"] # [doc = ""] # [doc = " assert_eq!(Ordering::Less, five.cmp(&Arc::new(6)));"] # [doc = " ```"] fn cmp (& self , other : & Self) -> cmp :: Ordering { (* * self) . cmp (& * * other) } }
};
}
