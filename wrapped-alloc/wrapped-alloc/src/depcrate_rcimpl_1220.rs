// Generated macro for impl_1220 (impl)
macro_rules! Depcrate_rcimpl_1220 {
() => {
// Module: crate::rc
// Provides: {"impl_1220"}
// Dependencies: {}
impl < T : Clone , A : Allocator > Rc < T , A > { # [doc = " If we have the only reference to `T` then unwrap it. Otherwise, clone `T` and return the"] # [doc = " clone."] # [doc = ""] # [doc = " Assuming `rc_t` is of type `Rc<T>`, this function is functionally equivalent to"] # [doc = " `(*rc_t).clone()`, but will avoid cloning the inner value where possible."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::{ptr, rc::Rc};"] # [doc = " let inner = String::from(\"test\");"] # [doc = " let ptr = inner.as_ptr();"] # [doc = ""] # [doc = " let rc = Rc::new(inner);"] # [doc = " let inner = Rc::unwrap_or_clone(rc);"] # [doc = " // The inner value was not cloned"] # [doc = " assert!(ptr::eq(ptr, inner.as_ptr()));"] # [doc = ""] # [doc = " let rc = Rc::new(inner);"] # [doc = " let rc2 = rc.clone();"] # [doc = " let inner = Rc::unwrap_or_clone(rc);"] # [doc = " // Because there were 2 references, we had to clone the inner value."] # [doc = " assert!(!ptr::eq(ptr, inner.as_ptr()));"] # [doc = " // `rc2` is the last reference, so when we unwrap it we get back"] # [doc = " // the original `String`."] # [doc = " let inner = Rc::unwrap_or_clone(rc2);"] # [doc = " assert!(ptr::eq(ptr, inner.as_ptr()));"] # [doc = " ```"] # [inline] # [stable (feature = "arc_unwrap_or_clone" , since = "1.76.0")] pub fn unwrap_or_clone (this : Self) -> T { Rc :: try_unwrap (this) . unwrap_or_else (| rc | (* rc) . clone ()) } }
};
}
