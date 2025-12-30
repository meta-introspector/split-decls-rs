// Generated macro for impl_1607 (impl)
macro_rules! Depcrate_syncimpl_1607 {
() => {
// Module: crate::sync
// Provides: {"impl_1607"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + PartialEq , A : Allocator > PartialEq for Arc < T , A > { # [doc = " Equality for two `Arc`s."] # [doc = ""] # [doc = " Two `Arc`s are equal if their inner values are equal, even if they are"] # [doc = " stored in different allocation."] # [doc = ""] # [doc = " If `T` also implements `Eq` (implying reflexivity of equality),"] # [doc = " two `Arc`s that point to the same allocation are always equal."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::Arc;"] # [doc = ""] # [doc = " let five = Arc::new(5);"] # [doc = ""] # [doc = " assert!(five == Arc::new(5));"] # [doc = " ```"] # [inline] fn eq (& self , other : & Arc < T , A >) -> bool { ArcEqIdent :: eq (self , other) } # [doc = " Inequality for two `Arc`s."] # [doc = ""] # [doc = " Two `Arc`s are not equal if their inner values are not equal."] # [doc = ""] # [doc = " If `T` also implements `Eq` (implying reflexivity of equality),"] # [doc = " two `Arc`s that point to the same value are always equal."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::Arc;"] # [doc = ""] # [doc = " let five = Arc::new(5);"] # [doc = ""] # [doc = " assert!(five != Arc::new(6));"] # [doc = " ```"] # [inline] fn ne (& self , other : & Arc < T , A >) -> bool { ArcEqIdent :: ne (self , other) } }
};
}
