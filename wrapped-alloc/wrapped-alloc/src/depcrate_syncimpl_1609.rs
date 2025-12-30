// Generated macro for impl_1609 (impl)
macro_rules! Depcrate_syncimpl_1609 {
() => {
// Module: crate::sync
// Provides: {"impl_1609"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + Ord , A : Allocator > Ord for Arc < T , A > { # [doc = " Comparison for two `Arc`s."] # [doc = ""] # [doc = " The two are compared by calling `cmp()` on their inner values."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::Arc;"] # [doc = " use std::cmp::Ordering;"] # [doc = ""] # [doc = " let five = Arc::new(5);"] # [doc = ""] # [doc = " assert_eq!(Ordering::Less, five.cmp(&Arc::new(6)));"] # [doc = " ```"] fn cmp (& self , other : & Arc < T , A >) -> Ordering { (* * self) . cmp (& * * other) } }
};
}
