// Generated macro for impl_1311 (impl)
macro_rules! Depcrate_rcimpl_1311 {
() => {
// Module: crate::rc
// Provides: {"impl_1311"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized + Ord , A : Allocator > Ord for UniqueRc < T , A > { # [doc = " Comparison for two `UniqueRc`s."] # [doc = ""] # [doc = " The two are compared by calling `cmp()` on their inner values."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(unique_rc_arc)]"] # [doc = " use std::rc::UniqueRc;"] # [doc = " use std::cmp::Ordering;"] # [doc = ""] # [doc = " let five = UniqueRc::new(5);"] # [doc = ""] # [doc = " assert_eq!(Ordering::Less, five.cmp(&UniqueRc::new(6)));"] # [doc = " ```"] # [inline] fn cmp (& self , other : & UniqueRc < T , A >) -> Ordering { (* * self) . cmp (& * * other) } }
};
}
