// Generated macro for impl_1663 (impl)
macro_rules! Depcrate_syncimpl_1663 {
() => {
// Module: crate::sync
// Provides: {"impl_1663"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized + Ord , A : Allocator > Ord for UniqueArc < T , A > { # [doc = " Comparison for two `UniqueArc`s."] # [doc = ""] # [doc = " The two are compared by calling `cmp()` on their inner values."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(unique_rc_arc)]"] # [doc = " use std::sync::UniqueArc;"] # [doc = " use std::cmp::Ordering;"] # [doc = ""] # [doc = " let five = UniqueArc::new(5);"] # [doc = ""] # [doc = " assert_eq!(Ordering::Less, five.cmp(&UniqueArc::new(6)));"] # [doc = " ```"] # [inline] fn cmp (& self , other : & UniqueArc < T , A >) -> Ordering { (* * self) . cmp (& * * other) } }
};
}
