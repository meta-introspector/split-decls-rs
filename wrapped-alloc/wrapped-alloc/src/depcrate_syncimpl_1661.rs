// Generated macro for impl_1661 (impl)
macro_rules! Depcrate_syncimpl_1661 {
() => {
// Module: crate::sync
// Provides: {"impl_1661"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized + PartialEq , A : Allocator > PartialEq for UniqueArc < T , A > { # [doc = " Equality for two `UniqueArc`s."] # [doc = ""] # [doc = " Two `UniqueArc`s are equal if their inner values are equal."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(unique_rc_arc)]"] # [doc = " use std::sync::UniqueArc;"] # [doc = ""] # [doc = " let five = UniqueArc::new(5);"] # [doc = ""] # [doc = " assert!(five == UniqueArc::new(5));"] # [doc = " ```"] # [inline] fn eq (& self , other : & Self) -> bool { PartialEq :: eq (& * * self , & * * other) } }
};
}
