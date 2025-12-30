// Generated macro for impl_1309 (impl)
macro_rules! Depcrate_rcimpl_1309 {
() => {
// Module: crate::rc
// Provides: {"impl_1309"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized + PartialEq , A : Allocator > PartialEq for UniqueRc < T , A > { # [doc = " Equality for two `UniqueRc`s."] # [doc = ""] # [doc = " Two `UniqueRc`s are equal if their inner values are equal."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(unique_rc_arc)]"] # [doc = " use std::rc::UniqueRc;"] # [doc = ""] # [doc = " let five = UniqueRc::new(5);"] # [doc = ""] # [doc = " assert!(five == UniqueRc::new(5));"] # [doc = " ```"] # [inline] fn eq (& self , other : & Self) -> bool { PartialEq :: eq (& * * self , & * * other) } # [doc = " Inequality for two `UniqueRc`s."] # [doc = ""] # [doc = " Two `UniqueRc`s are not equal if their inner values are not equal."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(unique_rc_arc)]"] # [doc = " use std::rc::UniqueRc;"] # [doc = ""] # [doc = " let five = UniqueRc::new(5);"] # [doc = ""] # [doc = " assert!(five != UniqueRc::new(6));"] # [doc = " ```"] # [inline] fn ne (& self , other : & Self) -> bool { PartialEq :: ne (& * * self , & * * other) } }
};
}
