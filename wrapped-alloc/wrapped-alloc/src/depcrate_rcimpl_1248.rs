// Generated macro for impl_1248 (impl)
macro_rules! Depcrate_rcimpl_1248 {
() => {
// Module: crate::rc
// Provides: {"impl_1248"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + PartialEq , A : Allocator > PartialEq for Rc < T , A > { # [doc = " Equality for two `Rc`s."] # [doc = ""] # [doc = " Two `Rc`s are equal if their inner values are equal, even if they are"] # [doc = " stored in different allocation."] # [doc = ""] # [doc = " If `T` also implements `Eq` (implying reflexivity of equality),"] # [doc = " two `Rc`s that point to the same allocation are"] # [doc = " always equal."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::rc::Rc;"] # [doc = ""] # [doc = " let five = Rc::new(5);"] # [doc = ""] # [doc = " assert!(five == Rc::new(5));"] # [doc = " ```"] # [inline] fn eq (& self , other : & Rc < T , A >) -> bool { RcEqIdent :: eq (self , other) } # [doc = " Inequality for two `Rc`s."] # [doc = ""] # [doc = " Two `Rc`s are not equal if their inner values are not equal."] # [doc = ""] # [doc = " If `T` also implements `Eq` (implying reflexivity of equality),"] # [doc = " two `Rc`s that point to the same allocation are"] # [doc = " always equal."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::rc::Rc;"] # [doc = ""] # [doc = " let five = Rc::new(5);"] # [doc = ""] # [doc = " assert!(five != Rc::new(6));"] # [doc = " ```"] # [inline] fn ne (& self , other : & Rc < T , A >) -> bool { RcEqIdent :: ne (self , other) } }
};
}
