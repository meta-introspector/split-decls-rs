// Generated macro for impl_1251 (impl)
macro_rules! Depcrate_rcimpl_1251 {
() => {
// Module: crate::rc
// Provides: {"impl_1251"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + Ord , A : Allocator > Ord for Rc < T , A > { # [doc = " Comparison for two `Rc`s."] # [doc = ""] # [doc = " The two are compared by calling `cmp()` on their inner values."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::rc::Rc;"] # [doc = " use std::cmp::Ordering;"] # [doc = ""] # [doc = " let five = Rc::new(5);"] # [doc = ""] # [doc = " assert_eq!(Ordering::Less, five.cmp(&Rc::new(6)));"] # [doc = " ```"] # [inline] fn cmp (& self , other : & Rc < T , A >) -> Ordering { (* * self) . cmp (& * * other) } }
};
}
