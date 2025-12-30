// Generated macro for impl_54 (impl)
macro_rules! Depcrate_arcimpl_54 {
() => {
// Module: crate::arc
// Provides: {"impl_54"}
// Dependencies: {}
impl < T : ? Sized + PartialEq > PartialEq for Arc < T > { # [doc = " Equality for two `Arc`s."] # [doc = ""] # [doc = " Two `Arc`s are equal if their inner values are equal, even if they are"] # [doc = " stored in different allocation."] # [doc = ""] # [doc = " If `T` also implements `Eq` (implying reflexivity of equality),"] # [doc = " two `Arc`s that point to the same allocation are always equal."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = ""] # [doc = " let five = Arc::new(5);"] # [doc = ""] # [doc = " assert!(five == Arc::new(5));"] # [doc = " ```"] # [inline] fn eq (& self , other : & Self) -> bool { * * self == * * other } # [doc = " Inequality for two `Arc`s."] # [doc = ""] # [doc = " Two `Arc`s are not equal if their inner values are not equal."] # [doc = ""] # [doc = " If `T` also implements `Eq` (implying reflexivity of equality),"] # [doc = " two `Arc`s that point to the same value are always equal."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = ""] # [doc = " let five = Arc::new(5);"] # [doc = ""] # [doc = " assert!(five != Arc::new(6));"] # [doc = " ```"] # [allow (clippy :: partialeq_ne_impl)] # [inline] fn ne (& self , other : & Self) -> bool { * * self != * * other } }
};
}
